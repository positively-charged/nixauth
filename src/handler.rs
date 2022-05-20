use std::net::SocketAddr;
use crate::{
   error::*,
   requests::{ NegotiateRequest, EphemeralRequest, ProofRequest },
   response::{ NegotiateResponse, EphemeralResponse, ProofResponse },
   storage::*,
   session::SessionList,
   session,
};

// For now, we do not support any protocol version below 2.
const MIN_SUPPORTED_PROTOCOL_VERSION: u8 = 2;

pub struct Handler<'storage> {
   storage: &'storage mut dyn Storage,
   sessions: &'storage mut SessionList<'storage>,
}

impl<'storage> Handler<'storage> {
   pub fn new( storage: &'storage mut dyn Storage,
      sessions: &'storage mut SessionList<'storage> ) -> Self {
      Handler { storage, sessions }
   }

   pub fn handle_negotiate( &mut self, remote_addr: SocketAddr,
      request: &NegotiateRequest ) -> Result<NegotiateResponse, ErrorResponse> {
      if ! ( request.protocol >= MIN_SUPPORTED_PROTOCOL_VERSION ) {
         return Err( ErrorResponse::User( UserError {
            client_session_id: request.client_session_id,
            error: ERR_USER_UNSUPPORTED_PROTOCOL,
         } ) );
      }

      let user = self.storage
         .get_user( request.username )
         .ok_or( ErrorResponse::User( UserError {
            error: ERR_USER_NOT_FOUND,
            client_session_id: request.client_session_id,
         } ) )?;

      let session_id = self.sessions
         .create_session( remote_addr )
         .ok_or( ErrorResponse::User( UserError {
            error: ERR_USER_BUSY,
            client_session_id: request.client_session_id,
         } ) )?;

      let mut session = session::Data::default();
      session.user_id = user.id;
      session.client_session_id = request.client_session_id;
      self.sessions.set_session_data( remote_addr, session_id, session );

      Ok( NegotiateResponse {
         client_session_id: request.client_session_id,
         session_id: session_id,
         salt: user.salt,
         username: user.username,
      } )
   }

   pub fn handle_ephemeral( &mut self, remote_addr: SocketAddr,
      request: &EphemeralRequest) -> Result<EphemeralResponse, ErrorResponse> {
      let mut session = self.sessions
         .get_session_data( remote_addr, request.session_id )
         .ok_or( ErrorResponse::Session( SessionError {
            error: ERR_SESSION_NOT_FOUND,
            session_id: request.session_id,
         } ) )?;

      // The user being deleted during authentication is highly unlikely, but
      // anticipate the condition nontheless.
      let user = self.storage
         .get_user_by_id( session.user_id as i64 )
         .ok_or( ErrorResponse::User( UserError {
            error: ERR_USER_NOT_FOUND,
            client_session_id: session.client_session_id,
         } ) )?;

      let mut authentication = crsrp::verifier::Verifier::new(
         crsrp::HashAlgorithm::Sha256, crsrp::Group::Bits2048,
         user.username.as_bytes(),
         user.verifier.as_slice(),
         user.salt.as_slice() );
      let server_public_key = authentication
         .create_public_key( &request.ephemeral )
         .map_err( |err| {
            println!("Verifier SRP-6a safety check violated!");
            ErrorResponse::Session( SessionError {
               error: ERR_SESSION_VERIFIER_UNSAFE,
               session_id: request.session_id,
            } )
         } )?;

      session.verifier = Some( authentication );
      session.client_public_key = request.ephemeral.to_vec();
      session.server_public_key = server_public_key.clone();
      self.sessions.set_session_data( remote_addr, request.session_id, session );
/*
      let mut session = self.sessions
         .get_session_data( request.session_id )
         .ok_or( ErrorResponse::Session( SessionError {
            error: ERR_SESSION_NOT_FOUND,
            session_id: request.session_id,
         } ) )?;
         */
      Ok( EphemeralResponse {
         session_id: request.session_id,
         ephemeral: server_public_key,
      } )
   }

   pub fn handle_proof( &mut self, remote_addr: SocketAddr,
      request: &ProofRequest ) -> Result<ProofResponse, ErrorResponse> {
      let mut session = self.sessions
         .get_session_data( remote_addr, request.session_id )
         .ok_or( ErrorResponse::Session( SessionError {
            error: ERR_SESSION_NOT_FOUND,
            session_id: request.session_id,
         } ) )?;

      // Just like for the ephemeral request, the user might be deleted during
      // authentication; although highly unlikely, anticipate the condition
      // nontheless.
      let user = self.storage
         .get_user_by_id( session.user_id as i64 )
         .ok_or( ErrorResponse::User( UserError {
            error: ERR_USER_NOT_FOUND,
            client_session_id: session.client_session_id,
         } ) )?;

      let authentication = session.verifier.take().unwrap();
      let ( key, proof ) = authentication
         .verify_session( request.proof )
         .map_err( |err| {
            println!("User authentication failed!");
            ErrorResponse::Session( SessionError {
               error: ERR_SESSION_FAILURE,
               session_id: request.session_id,
            } )
         } )?;

      self.sessions.destroy( request.session_id );

      Ok( ProofResponse {
         session_id: request.session_id,
         proof: proof,
      } )
   }
}
