use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Instant;
use std::time::Duration;

use rand::Rng;
use crsrp::verifier::Verifier;

use crate::log::Log;

// Same as the maximum number of clients in a Zandronum server.
const MAX_SESSIONS_PER_ORIGIN: u32 = 64;
const MAX_SESSIONS: u32 = MAX_SESSIONS_PER_ORIGIN * 4;

struct Origin {
   sessions: Vec<Session>
}

struct Handle {
   pub session_id: u32,
   pub origin: SocketAddr,
}

/**

   Each session contains data that can be manipulated.

*/
#[ derive( Default ) ]
#[ derive( Debug ) ]
pub struct Data {
   pub user_id: usize,
   pub client_session_id: u32,
   pub client_public_key: Vec<u8>,
   pub server_public_key: Vec<u8>,
   pub verifier: Option<Verifier>,
}

struct Session {
   id: i16,
   data: Option<Data>,
   last_updated: Instant,
   owner: Option<SocketAddr>,
}

impl Default for Session {
   fn default() -> Self {
      Session {
         id: i16::default(),
         data: Option::<Data>::default(),
         last_updated: Instant::now(),
         owner: None,
      }
   }
}

pub struct SessionList<'log> {
   entries: Vec<Session>,
  // origins: HashMap<SocketAddrV4, Origin>,
   max_entries: i16,
   used_entries: i16,
   next_free_entry: i16,
   session_lifetime: i16,
   log: &'log Log,
   session_mask: u32,
   //subranges_to_entries: HashMap<u16, u16>,
}

impl<'log> SessionList<'log> {
   pub fn new( log: &'log Log, max_entries: i16, session_lifetime: i16 ) -> Self {
      Self {
         entries: Vec::with_capacity( 4 ),
         max_entries,
         used_entries: 0,
         next_free_entry: -1,
         session_lifetime,
         log,
         session_mask: rand::thread_rng().gen::<u32>(),
      }
      // hashed_index = random_number - random_number % 256 + index
      // hashed_index - random_number = - random_number % 256 + index
      // hashed_index - random_number + random_number % 256 = index
      // hashed_index % 256 - random_number + random_number = index
   }

   pub fn create_session( &mut self, owner: SocketAddr ) -> Option<u32> {
      if self.used_entries < self.max_entries {
         let session_index = self.alloc_session();
         let session_id = self.encode_session_id( session_index );
         let session = &mut self.entries[ session_index ];
         session.last_updated = Instant::now();
         session.owner = Some( owner );
         self.used_entries += 1;
         return Some( session_id );
        // println!( "{}", self.generate_session_id() );
        // println!( "{:?}", self.entries );
        // if origin.total_entries == self.entries.len() {
        //    self.entries.push();
        // }
      }
      None
   }

   fn alloc_session( &mut self ) -> usize {
      // Use a previously allocated session object from the free list.
      if self.next_free_entry >= 0 {
         let index = self.next_free_entry as usize;
         let session = &mut self.entries[ index ];
         self.next_free_entry = session.id;
         index
      }
      else {
         let index = self.entries.len();
         // Double the size of the buffer.
         self.entries.reserve( self.entries.len() );
         self.entries.push( Session::default() );
         index
      }
   }

   fn get_timestamp() -> u64 {
      std::time::SystemTime::now()
         .duration_since( std::time::UNIX_EPOCH )
         .unwrap_or( std::time::Duration::from_secs( 0 ) )
         .as_secs()
   }

   fn decode_session_id( &self, session_id: u32 ) -> usize {
      let session_id = session_id ^ self.session_mask;
      let encoded_index = session_id % self.max_entries as u32;
      let start_of_interval = session_id - encoded_index;
      let mask = self.session_mask.wrapping_add( start_of_interval /
         self.max_entries as u32 );
      let shift = mask % self.max_entries as u32;
      let index = ( encoded_index.wrapping_sub( shift ) ) %
         self.max_entries as u32;
      let index = ( self.max_entries as u32 - shift + encoded_index ) %
         self.max_entries as u32;
      index as usize
   }

   // We utilize the whole range of u32.
   fn encode_session_id( &self, index: usize ) -> u32 {
      let random_number = rand::thread_rng().gen::<u32>();
      // Bring the random number to the start of the interval that the random
      // number is in.
      let start_of_interval = random_number -
         random_number % self.max_entries as u32;
      let mask = self.session_mask.wrapping_add( start_of_interval /
         self.max_entries as u32 );
      let shift_amount = mask % self.max_entries as u32;
      let encoded_index = ( shift_amount + index  as u32 ) %
         self.max_entries as u32;
     ( start_of_interval + encoded_index ) ^ self.session_mask
   }

   fn get_session( &mut self, remote_addr: SocketAddr,
      session_id: u32 ) -> Option<&mut Session> {
      let session = self.decode_session_id( session_id );
      if session < self.entries.len() &&
         ! self.is_expired_session( session ) &&
         self.is_session_owned_by( session, remote_addr ) {
         return Some( &mut self.entries[ session ] );
      }
      None
   }

   fn is_session_owned_by( &self, session: usize,
      remote_addr: SocketAddr ) -> bool {
      if let Some( owner ) = self.entries[ session ].owner {
         return owner == remote_addr;
      }
      false
   }

   pub fn get_session_data( &mut self, remote_addr: SocketAddr,
      id: u32 ) -> Option<Data> {
      if let Some( session ) = self.get_session( remote_addr, id ) {
         return session.data.take();
      }
      None
   }

   pub fn set_session_data( &mut self, remote_addr: SocketAddr, id: u32,
      data: Data ) -> bool {
      if let Some( session ) = self.get_session( remote_addr, id ) {
         session.data = Some( data );
         session.last_updated = Instant::now();
         return true;
      }
      false
   }

   fn is_expired_session( &mut self, index: usize ) -> bool {
      self.entries[ index ].last_updated.elapsed().as_secs() >=
         self.session_lifetime as u64
   }

   /**

      Removes any expired sessions.

   */
   pub fn clean( &mut self ) {
      for entry in 0 .. self.entries.len() {
         if self.is_expired_session( entry ) {
            self.destroy( entry as u32 );
         }
      }
      self.log.write( &format!( "used-entries: {}", self.used_entries ) );
      self.log.write( &format!( "reserved-entries: {}", self.entries.len() ) );
      //self.log.write( LOG_DBG, "line {}" ).arg( 123 )
   }

   pub fn destroy( &mut self, session_id: u32 ) {
      if ( session_id as usize ) < self.entries.len() {
         let id = self.entries[ session_id as usize ].id;
         self.entries[ session_id as usize  ].id = self.next_free_entry;
         self.next_free_entry = session_id as i16;
         self.used_entries -= 1;
      }
   }
}

#[ cfg( test ) ]
#[ path = "tests/session.rs" ]
mod test;
