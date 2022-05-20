use std::time::Instant;
use std::time::SystemTime;

use nixauth::log::Log;
use nixauth::session::SessionList;

#[ test ]
fn create_session() {
   let mut log = Log::new();
   let now = Instant::now();
   let mut list = SessionList::new( &log, 10_000, 1 );
   list.create_session();
   std::thread::sleep( std::time::Duration::from_secs( 1 ) );
   list.create_session();
   println!( "{:?}", now.elapsed().as_secs() );
}

#[ test ]
fn expired_session() {
   let mut log = Log::new();
   let mut list = SessionList::new( &log, 10_000, 1 );
   list.create_session();
   std::thread::sleep( std::time::Duration::from_secs( 1 ) );
   assert!( list.get_session_data( 0 ).is_none() );
}

#[ test ]
fn clear_expired_sessions() {
   let mut log = Log::new();
   let now = SystemTime::now();
   println!( "{:?}", now.duration_since( std::time::UNIX_EPOCH ).unwrap().as_secs() );
   let mut list = SessionList::new( &log, 10_000, 1 );
   list.create_session();
   std::thread::sleep( std::time::Duration::from_secs( 1 ) );
   list.create_session();
   println!( "{:?}",  SystemTime::now().duration_since( std::time::UNIX_EPOCH ).unwrap().as_secs() );
}

#[ test ]
fn clean() {
   let mut log = Log::new();
   log.write( &"abc" );

   let mut list = SessionList::new( &log, 10_000, 1 );
   list.create_session();
   list.create_session();
   list.create_session();
   std::thread::sleep( std::time::Duration::from_secs( 1 ) );
   list.clean();
}
