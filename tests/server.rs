use std::sync::Arc;
use std::sync::atomic::{ AtomicBool, Ordering };

use nixauth::server::Server;

#[ test ]
fn test_server() {
/*
   let done = Arc::new( AtomicBool::new( false ) );
   let done_copy = done.clone();
   ctrlc::set_handler( move || {
      done_copy.store( true, Ordering::SeqCst );
   } ).unwrap();
*/
   let server = Server::new( 16666 );
   server.run();
}
