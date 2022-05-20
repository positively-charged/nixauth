use std::time::Instant;
use std::time::SystemTime;
use std::time::Duration;
use std::thread::sleep;
use std::net::SocketAddr;

use crate::log::Log;
use crate::session::SessionList;
use crate::session::Data;

#[ test ]
fn create_session() {
   let test = Test::new();
   let mut list = test.session_list();
   list.create_session( dummy_addr() );
   assert!( list.entries.len() == 1 );
   assert!( list.used_entries == 1 );
}

fn dummy_addr() -> SocketAddr {
   SocketAddr::V4( "127.0.0.1:8080".parse().unwrap() )
}

fn dummy_addr2() -> SocketAddr {
   SocketAddr::V4( "192.168.0.1:10666".parse().unwrap() )
}

#[ test ]
fn encode_decode_session_id() {
   let test = Test::new().max_sessions( 256 );
   let mut list = test.session_list();
   let session_id = list.encode_session_id( 3 );
   println!( "encoded-session-id: {} {:x}", session_id, session_id );
   let decoded = list.decode_session_id( session_id );
   println!( "decoded-session-id: {} {:x}", decoded, decoded );
   let session_id = list.encode_session_id( 3 );
   println!( "encoded-session-id: {} {:x}", session_id, session_id );
   let decoded = list.decode_session_id( session_id );
   println!( "decoded-session-id: {} {:x}", decoded, decoded );
   println!( "{}", 128u8.rotate_left( 9u32 ) );


   let test = Test::new();
   let mut list = test.session_list();
   let session_id = list.encode_session_id( 255 );
   assert!( list.decode_session_id( session_id ) == 255 );

} // index=1 mask=2 max-entries=3 index^mask%max-entries=0
// encoded-index % max-entries = r
// encoded-index = q * max-entries + r
// mask ^  index = q * max-entries + r

// r = dividend - floor( dividend / divisor ) * divisor
// r = encoded-index - ( encoded-index / max-entries ) * max-entries

#[ test ]
fn get_valid_session_but_wrong_owner() {
   let test = Test::new();
   let mut sessions = test.session_list();
   let session_id = sessions.create_session( dummy_addr() ).unwrap();
   println!( "session-id: {}", session_id );
   assert!( sessions.set_session_data( dummy_addr(), session_id,
      Data::default() ) );
   assert!( sessions.get_session_data( dummy_addr(), session_id ).is_some() );
   assert!( sessions.get_session_data( dummy_addr2(), session_id ).is_none() );
}

#[ test ]
fn max_sessions_reached() {
   let test = Test::new().max_sessions( 1 );
   let mut list = test.session_list();
   assert!( list.create_session( dummy_addr() ).is_some() );
   assert!( list.create_session( dummy_addr() ).is_none() );
}

#[ test ]
fn expired_session() {
   let test = Test::new();
   let mut list = test.session_list();
   let session = list.create_session( dummy_addr() );
   assert!( list.is_expired_session( 0 ) == false );
   sleep( Duration::from_secs( 1 ) );
   assert!( list.is_expired_session( 0 ) == true );
}

#[ test ]
fn updating_session_data_extends_session_lifetime() {
   let test = Test::new();
   let mut list = test.session_list();
   let session = list.create_session( dummy_addr() ).unwrap();
   sleep( Duration::from_millis( 500 ) );
   list.set_session_data( dummy_addr(), session, Data::default() );
   sleep( Duration::from_millis( 500 ) );
   list.clean();
   assert!( list.used_entries == 1 );
}

struct Test {
   log: Log,
   max_sessions: i16,
}

impl Test {
   fn new() -> Self {
      Self { log: Log::new(), max_sessions: 256 }
   }

   fn max_sessions( mut self, max_sessions: i16 ) -> Self {
      self.max_sessions = max_sessions;
      self
   }

   fn session_list( &self ) -> SessionList {
      SessionList::new( &self.log, self.max_sessions, 1 )
   }
}
