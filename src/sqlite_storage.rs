use sqlite::Connection;
use sqlite::OpenFlags;
use sqlite::Bindable;
use crate::storage::Storage;
use crate::storage::{ Session, User };

pub struct SqliteStorage {
   db: Connection,
}

pub struct Row<'storage> {
   stmt: &'storage sqlite::Statement<'storage>,
}

impl<'storage> Row<'storage> {
   fn new( stmt: &'storage sqlite::Statement<'storage> ) -> Self {
      Row { stmt }
   }

   fn unwrap_integer( &self, column: usize ) -> i64 {
      self.stmt.read::<i64>( column ).unwrap()
   }

   fn unwrap_string( &self, column: usize ) -> String {
      self.stmt.read::<String>( column ).unwrap()
   }

   fn unwrap_blob( &self, column: usize ) -> Vec<u8> {
      self.stmt.read::<Vec<u8>>( column ).unwrap()
   }
}

pub struct Query<'storage> {
   db: &'storage Connection,
   stmt: sqlite::Statement<'storage>,
}

impl<'storage> Query<'storage> {
   fn new( db: &'storage Connection, query: &str ) -> Self {
      let stmt = db.prepare( query ).unwrap();
      Query { db, stmt }
   }

   fn bind_by_name<T: Bindable>( mut self, name: &str, value: T ) -> Self {
      self.stmt.bind_by_name( name, value ).unwrap();
      self
   }

   fn fetch_row<F, T>( mut self, callback: F ) -> Option<T>
      where F: Fn( Row ) -> T {
      if let Ok( state ) = self.stmt.next() {
         if state == sqlite::State::Row {
            return Some( callback( Row { stmt: &self.stmt } ) );
         }
      }
      None
   }

   fn execute( mut self ) -> bool {
      if let Ok( state ) = self.stmt.next() {
         return state == sqlite::State::Done;
      }
      false
   }
}

impl SqliteStorage {
   pub fn new( path: &str ) -> Self {
      let db = match Connection::open_with_flags( path,
         OpenFlags::new().set_read_write() ) {
         Ok( db ) => db,
         Err( .. ) => panic!( "failed to open database file: {}", path )
      };


      let contents = match std::fs::read_to_string( "db/db.sql" ) {
         Ok( contents ) => contents,
         Err( .. ) =>
            panic!( "can't open db initialization file" )
      };

      db.execute( &contents ).unwrap();

      SqliteStorage { db }
   }

   pub fn new_in_memory() -> Self {
      let db = match Connection::open_with_flags( ":memory:",
         OpenFlags::new().set_read_write() ) {
         Ok( db ) => db,
         Err( .. ) => panic!( "failed to open in-memory database" )
      };

      let contents = match std::fs::read_to_string( "db/db.sql" ) {
         Ok( contents ) => contents,
         Err( .. ) =>
            panic!( "can't open db initialization file" )
      };

      db.execute( &contents ).unwrap();

      SqliteStorage { db }
   }

   pub fn open( path: &str ) -> Self {
      let db = match Connection::open_with_flags( path,
         OpenFlags::new().set_read_write() ) {
         Ok( db ) => db,
         Err( .. ) => panic!( "failed to open database file: {}", path )
      };
      SqliteStorage { db }
   }

   pub fn add_user( &self, username: &str, verifier: &[ u8 ],
      salt: &[ u8 ] ) -> bool {
      self.prepare(
         "INSERT INTO user VALUES ( :username, :verifier, :salt )" )
         .bind_by_name( ":username", username )
         .bind_by_name( ":verifier", verifier )
         .bind_by_name( ":salt", salt )
         .execute()
   }

   fn prepare( &self, query: &str ) -> Query {
      Query::new( &self.db, query )
   }
}

impl Storage for SqliteStorage {
   fn get_user( &mut self, username: &str ) -> Option<User> {
      self.prepare( concat!(
         "SELECT rowid, username, verifier, salt FROM user ",
         "WHERE username = :username" ) )
         .bind_by_name( ":username", username )
         .fetch_row( |row| { User {
            id: row.unwrap_integer( 0 ) as usize,
            username: row.unwrap_string( 1 ),
            verifier: row.unwrap_blob( 2 ),
            salt: row.unwrap_blob( 3 ),
         } } )
   }

   fn get_user_by_id( &mut self, id: i64 ) -> Option<User> {
      self.prepare( concat!(
         "SELECT rowid, username, verifier, salt FROM user ",
         "WHERE rowid = :id" ) )
         .bind_by_name( ":id", id )
         .fetch_row( |row| { User {
            id: row.unwrap_integer( 0 ) as usize,
            username: row.unwrap_string( 1 ),
            verifier: row.unwrap_blob( 2 ),
            salt: row.unwrap_blob( 3 ),
         } } )
   }

   fn add_user( &mut self, username: &str, verifier: &[ u8 ],
      salt: &[ u8 ] ) -> bool {
      SqliteStorage::add_user( self, username, verifier, salt )
   }

   fn remove_user( &mut self, username: &str ) -> bool {
      if ! self.prepare( concat!(
         "DELETE FROM user WHERE username = :username" ) )
         .bind_by_name( ":username", username )
         .execute() {
         return false;
      }
      self.db.change_count() == 1
   }

/*
   fn create_session( &mut self, user: &User,
      client_session_id: u32 ) -> Option<u32> {
      self.prepare( concat!(
         "INSERT INTO session ( id, user_id, client_session_id ) ",
         "VALUES ( :id, :user_id, :client_session_id )" ) )
         .bind_by_name( ":id", 12345 )
         .bind_by_name( ":user_id", user.row_id as i64 )
         .bind_by_name( ":client_session_id", client_session_id as i64 )
         .execute()
         .then( || 12345u32 )
   }

   fn get_session_by_id( &mut self, id: u32 ) -> Option<Session> {
      self.prepare( concat!(
         "SELECT user_id, client_session_id, ephemeral_a, ephemeral_b ",
         "FROM session WHERE id = :id" ) )
         .bind_by_name( ":id", id as i64 )
         .fetch_row( |row| { Session {
            id: id,
            user_id: row.unwrap_integer( 0 ) as i32,
            client_session_id: row.unwrap_integer( 1 ) as u32,
            ephemeral_a: row.unwrap_blob( 2 ),
            ephemeral_b: row.unwrap_blob( 3 ),
         } } )
   }

   fn set_ephemeral( &mut self, session: &Session, ephemeral_a: &[ u8 ],
      ephemeral_b: &[ u8 ] ) -> bool {
      self.prepare( concat!(
         "UPDATE session SET ",
         "ephemeral_a = :ephemeral_a, ",
         "ephemeral_b = :ephemeral_b ",
         "WHERE id = :id" ) )
         .bind_by_name( ":ephemeral_a", ephemeral_a )
         .bind_by_name( ":ephemeral_b", ephemeral_b )
         .bind_by_name( ":id", session.id as i64 )
         .execute()
   }*/
}
