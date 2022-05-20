use crate::writer::Writer;
use crate::reader::Reader;

// Messages we send back to the client.
pub const RESPONSE_NEGOTIATE: u32 = 0xD003CA10;
pub const RESPONSE_EPHEMERAL: u32 = 0xD003CA20;
pub const RESPONSE_PROOF: u32 = 0xD003CA30;

pub struct Response {

}

/*
impl Response {
   pub fn addU32( self, value: u32 ) {}
   pub fn addStr( self, value: &str ) {}
}*/

#[ derive( Debug ) ]
pub struct NegotiateResponse {
   pub client_session_id: u32,
   pub session_id: u32,
   pub salt: Vec<u8>,
   pub username: String,
}

impl NegotiateResponse {
   pub fn marshall( &self, writer: &mut Writer ) {
      writer.append_u32le( RESPONSE_NEGOTIATE );
      // Not sure what the `protocol` field is supposed to be in the original
      // charonauth code base. The charonauth sets it to 1, so we will do the
      // same thing. We will also ignore this field for now.
      writer.append_u8( 1 );
      writer.append_u32le( self.client_session_id );
      writer.append_u32le( self.session_id );
      writer.append_u8( self.salt.len() as u8 );
      writer.append_bytes( self.salt.as_slice() );
      writer.append_bytes( self.username.as_bytes() );
      writer.append_u8( 0 );
   }

   pub fn unmarshall( reader: &mut Reader ) -> Result<Self, ()> {
      let _protocol = reader.read_u8()?;

      let client_session_id = reader.read_u32le()?;
      let session_id = reader.read_u32le()?;
      let salt_length = reader.read_u8()?;
      let ( reader, salt ) = reader.read_bytes( salt_length.into() );
      let salt = salt?;
      let username = match reader.read_ascii_str() {
         ( _, username ) => username?
      };
      Ok( Self { client_session_id, session_id,
         salt: salt.to_vec(), username: username.to_string() } )
   }
}

#[ derive( Debug ) ]
pub struct EphemeralResponse {
   pub session_id: u32,
   pub ephemeral: Vec<u8>,
}

impl EphemeralResponse {
   pub fn marshall( &self, writer: &mut Writer ) {
      writer.append_u32le( RESPONSE_EPHEMERAL );
      writer.append_u32le( self.session_id );
      writer.append_u16le( self.ephemeral.len() as u16 );
      writer.append_bytes( self.ephemeral.as_slice() );
   }

   pub fn unmarshall( reader: &mut Reader ) -> Result<Self, ()> {
      let session_id = reader.read_u32le()?;
      let ephemeral_length = reader.read_u16le()?;
      let ( _, ephemeral ) = reader.read_bytes( ephemeral_length as usize );
      let ephemeral = ephemeral?.to_vec();
      Ok( Self { session_id, ephemeral } )
   }
}

#[ derive( Debug ) ]
pub struct ProofResponse {
   pub session_id: u32,
   pub proof: Vec<u8>,
}

impl ProofResponse {
   pub fn marshall( &self, writer: &mut Writer ) {
      writer.append_u32le( RESPONSE_PROOF );
      writer.append_u32le( self.session_id );
      writer.append_u16le( self.proof.len() as u16 );
      writer.append_bytes( self.proof.as_slice() );
   }

   pub fn unmarshall( reader: & mut Reader ) -> Result<Self, ()> {
      let session_id = reader.read_u32le()?;
      let proof_length = reader.read_u16le()?;
      let ( _, proof ) = reader.read_bytes( proof_length as usize );
      let proof = proof?.to_vec();
      Ok( Self { session_id, proof } )
   }
}
