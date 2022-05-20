use std::net::SocketAddr;

use crate::requests::*;
//use crate::response::*;
use crate::handler::Handler;
use crate::reader::*;
use crate::writer::*;

pub struct Receiver<'handler, 'storage: 'handler> {
   handler: &'handler mut Handler<'storage>,
}

impl<'handler, 'storage: 'handler> Receiver<'handler, 'storage> {
   pub fn new( handler: &'handler mut Handler<'storage> ) -> Self {
      Receiver { handler }
   }

   // If a request could not be decoded successfully, it will be silently
   // dropped.
   pub fn handle_request( &mut self, remote_addr: SocketAddr,
      content: &mut Reader, output: &mut Writer ) -> bool {
      if let Ok( mut request ) = Request::from_reader( content ) {
         return match request.header {
            REQUEST_NEGOTIATE => self.on_negotiate( remote_addr, &mut request, output ),
            REQUEST_EPHEMERAL => self.on_ephemeral( remote_addr, &mut request, output ),
            REQUEST_PROOF => self.on_proof( remote_addr, &mut request, output ),
            _ => false,
         };
      }
      false
   }

   fn on_negotiate( &mut self, remote_addr: SocketAddr, request: &mut Request,
      output: &mut Writer ) -> bool {
      NegotiateRequest::unmarshall( request )
         .map( |request| match self.handler.handle_negotiate( remote_addr, &request ) {
            Ok( response ) => response.marshall( output ),
            Err( response ) => response.marshall( output )
         } ).is_ok()
   }

   fn on_ephemeral( &mut self, remote_addr: SocketAddr, request: &mut Request,
      output: &mut Writer ) -> bool {
      // If the request could not be decoded, drop it silently.
      if let Ok( request ) = EphemeralRequest::unmarshall( request ) {
         match self.handler.handle_ephemeral( remote_addr, &request ) {
            Ok( response ) => response.marshall( output ),
            Err( response ) => {
               println!( "{:?}", response );
            }
         }
         return true;
      }
      false
   }

   fn on_proof( &mut self, remote_addr: SocketAddr, request: &mut Request,
      output: &mut Writer ) -> bool {
      // If the request could not be decoded, drop it silently.
      if let Ok( request ) = ProofRequest::unmarshall( request ) {
      println!( "{:?}", request );
         match self.handler.handle_proof( remote_addr, &request ) {
            Ok( response ) => {
               println!( "{:?}", response );
               response.marshall( output );
            },
            Err( response ) => {
               response.marshall( output );
               println!( "{:?}", response );
            }
         }
         return true;
      }
      false
   }
}
