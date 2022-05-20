//use rand::RngCore;
//use crate::session::Session;

pub struct User {
   pub id: usize,
   pub username: String,
   pub verifier: Vec<u8>,
   pub salt: Vec<u8>,
}

pub struct Session {
   pub id: u32,
   pub user_id: i32,
   pub client_session_id: u32,
   pub ephemeral_a: Vec<u8>,
   pub ephemeral_b: Vec<u8>,
}

pub enum Err {
   OutOfSlots,
}

/*
pub trait User {
   pub fn add( &mut self,
      username: &str,
      verifier: &[ u8 ],
      salt: &[ u8 ] ) -> bool;
   pub fn get( &mut self, username: &str ) -> Option<User>;
}

pub trait Session {
   fn create( &mut self, user: &User ) -> Option<u32>;
   fn get( &mut self, id: u32 ) -> Option<Session>;
   fn get_user( &mut self, id: u32 ) -> Option<Session>;
   fn set_ephemeral( &mut self, ephemeral_a: &[ u8 ], ephemeral_b: &[ u8 ] ) -> bool;
} */

pub trait Storage {
   fn get_user( &mut self, username: &str ) -> Option<User>;
   fn get_user_by_id( &mut self, id: i64 ) -> Option<User>;
   fn add_user( &mut self, username: &str, verifier: &[ u8 ],
      salt: &[ u8 ] ) -> bool;
   fn remove_user( &mut self, username: &str ) -> bool;
}

pub static USERNAME: &str = "test-user";
pub static PASSWORD: &str = "12345";
pub static SALT: &str = "xxx";

/*
pub struct MyStorage {
   session: Option<Session>,
   //private_key: Vec<u8>,
   verifier: Vec<u8>,
}
impl MyStorage {
   pub fn new() -> Self {
      let private_key = srp_private_key::<Sha256>( USERNAME.as_bytes(),
         PASSWORD.as_bytes(), SALT.as_bytes() );

      let verifier = SrpClient::<Sha256>::new( &[], &G_2048 )
         .get_password_verifier( &private_key );

      MyStorage {
         session: None,
         //private_key: private_key.into_iter().collect(),
         verifier
      }
   }
}

impl Storage for MyStorage {
   fn get_user( &mut self, _username: &str ) -> Option<User> {
      Some( User {
         row_id: 0,
         username: USERNAME.to_string(),
         salt: SALT.as_bytes().to_vec(),
         verifier: self.verifier.clone()
         //verifier: SALT.as_bytes()
      } )
   }

   fn create_session( &mut self, user: &User ) ->
      Option<u32> {
      None /*
      if self.session.is_none() {
         let session = Session {
            id: 12345,
            username: USERNAME.as_bytes(),
            salt: SALT.as_bytes(),
            verifier: &self.verifier,
            ephemeral_a: "abc".as_bytes(),
         };
         //let session = Session::new( &self.private_key, &self.verifier );
         self.session = Some( session );
         Some( self.session.as_mut().unwrap() )
      }
      else {
         // Err( Err::OutOfSlots )
         None
      }*/
   }

   fn get_session( &mut self, id: u32 ) -> Option<Session> {
      None
      /*
      if let Some( ref mut session ) = self.session {
         if session.id == id {
            return Some( session );
         }
      }
      Some( Session {
         id,
         username: USERNAME.as_bytes(),
         salt: SALT.as_bytes(),
         verifier: &self.verifier,
         ephemeral_a: "abc".as_bytes(),
      } ) */
   }
}
*/
