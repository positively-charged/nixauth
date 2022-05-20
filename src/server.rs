use std::str::FromStr;
use std::net::SocketAddrV4;
use std::net::UdpSocket;
//use std::thread::sleep;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{ AtomicBool, Ordering };

use crate::handler::Handler;
use crate::receiver::Receiver;
use crate::writer::Writer;
use crate::reader::Reader;
use crate::session::SessionList;
use crate::log::Log;
use crate::storage::Storage;

//use crate::reader::Reader;
//use crate::writer::Writer;
//use crate::handler::Handler;

pub struct Server<'storage> {
   storage: &'storage mut dyn Storage,
   port: i32,
}

impl<'storage> Server<'storage> {
   pub fn new( storage: &'storage mut dyn Storage, port: i32 ) -> Self {
      Server { storage, port }
   }

   pub fn run( mut self ) {
      let done = Arc::new( AtomicBool::new( false ) );
      let done_copy = done.clone();
      ctrlc::set_handler( move || {
         done_copy.store( true, Ordering::SeqCst );
      } ).unwrap();

      let log = Log::new();

      let mut sessions = SessionList::new( &log, 3, 5 );
      let mut handler = Handler::new( self.storage, &mut sessions );

      let addr = SocketAddrV4::from_str( "127.0.0.1:16666" ).unwrap();
      let socket = UdpSocket::bind( addr ).unwrap();
      socket.set_read_timeout( Some( Duration::from_secs( 1 ) ) ).unwrap();
/*
      use crate::requests::NegotiateRequest;
      let request = NegotiateRequest {
         protocol: 2,
         client_session_id: 12345,
         username: "test"
      };
      let mut writer = Writer::new();
      request.marshall( &mut writer );
      for byte in writer.contents() {
         print!( "\\x{:02X}", byte );
      }
      println!();
      return;*/

      println!( "accepting connections" );

      let mut i = 0;
      while ! done.load( Ordering::SeqCst ) {
         Self::handle_request( &socket, &mut handler );
         i += 1;
      }

      println!( "shutting down" );

      return;


      /*
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

   fn handle_request(
      socket: &UdpSocket, handler: &mut Handler ) {
      let mut buffer = [ 0; 1000 ];
      match socket.recv_from( &mut buffer ) {
         Ok( ( bytes_read, remote_addr ) ) => {
               println!( "bytes read: {}", bytes_read );
               println!( "remote_addr: {}:{}", remote_addr.ip(), remote_addr.port() );

            let mut content = Reader::new( &buffer );
            let mut response = Writer::new();
            let mut receiver = Receiver::new( handler );
            if receiver.handle_request( remote_addr, &mut content, &mut response ) {
              // let mut reader = Reader::new( response.contents() );
              // reader.read_u32le().unwrap();
              // let response = crate::response::
              //    NegotiateResponse::unmarshall( &mut reader ).unwrap();
              assert!( socket.send_to( response.contents(),
                  remote_addr ).unwrap() == response.contents().len() );
               println!( "sending back {:?}", response.contents().len() );
            }
         }
         _ => {}
      }
   }

   pub fn add_user( &mut self, username: &[ u8 ], password: &[ u8 ] ) {

   }

   pub fn disable_user( &mut self, username: &[ u8 ] ) {}
   pub fn enable_user( &mut self, username: &[ u8 ] ) {}
   pub fn remove_user( &mut self, username: &[ u8 ] ) {}
}
