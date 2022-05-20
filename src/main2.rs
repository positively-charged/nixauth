use std::sync::Arc;
use std::sync::atomic::{ AtomicBool, Ordering };

use crate::Server;
use crate::sqlite_storage::SqliteStorage;

fn main() {
/*
   let done = Arc::new( AtomicBool::new( false ) );
   let done_copy = done.clone();
   ctrlc::set_handler( move || {
      done_copy.store( true, Ordering::SeqCst );
   } ).unwrap(); */

   let mut storage = SqliteStorage::open( "test.db" );
   let server = Server::new( &mut storage, 10666 );
   server.run();

/*
   let mut request = Request::new();
   println!( "{}", request.content.capacity() );

   let mut writer = Writer::new();
   writer.append_u32le( REQUEST_NEGOTIATE );
   writer.append_u8( 2 );
   writer.append_u32le( 123 );
   writer.append_bytes( b"positron\0" );
   request.content = writer.contents().to_vec();

   let server = Server::new();
   let response = server.handle_request( &mut request );
   response.unwrap();

   return;
   let mut request = Vec::<u8>::new();
   request.extend( REQUEST_NEGOTIATE.to_le_bytes() );
   request.extend( b"abc\0" );
   println!( "{}", request.len() );
   println!( "{}", request.capacity() );
   request[ 0..4 ].copy_from_slice( &REQUEST_NEGOTIATE.to_le_bytes() );
    let result = SocketAddrV4::from_str( "127.0.0.1:10666" );
    match result {
        Ok( addr ) => {
            println!( "{}", addr );
            println!( "is localhost: {}", addr.ip().is_loopback() );
            println!( "port: {}", addr.port() );
            let socket = UdpSocket::bind( addr );
        },
        Err( e ) => println!( "error: {}", e ),
    }
    */
}
