use crate::reader::Reader;
use crate::writer::Writer;

// Messages we receive from a client.
pub const REQUEST_NEGOTIATE: u32 = 0xD003CA01;
pub const REQUEST_EPHEMERAL: u32 = 0xD003CA02;
pub const REQUEST_PROOF: u32 = 0xD003CA03;

pub struct Request<'reader, 'content: 'reader> {
   reader: &'reader mut Reader<'content>,
   pub header: u32,
}

impl<'reader, 'content: 'reader> Request<'reader, 'content> {
   pub fn from_reader( reader: &'reader mut Reader<'content> ) -> Result<Self, ()> {
      let value = reader.read_u32le()?;
      let header = match value {
         REQUEST_NEGOTIATE |
         REQUEST_EPHEMERAL |
         REQUEST_PROOF => value,
         _ => return Err( () ),
      };
      Ok( Request { reader, header } )
   }
}

#[ derive( Debug ) ]
pub struct NegotiateRequest<'a> {
   pub protocol: u8,
   pub client_session_id: u32,
   pub username: &'a str,
}

impl<'a> NegotiateRequest<'a> {
   pub fn marshall( &self, writer: &mut Writer ) {
      writer.append_u32le( REQUEST_NEGOTIATE );
      writer.append_u8( self.protocol );
      writer.append_u32le( self.client_session_id );
      writer.append_bytes( self.username.as_bytes() );
      writer.append_u8( 0 );
   }

   pub fn unmarshall( request: &'a mut Request ) -> Result<Self, ()> {
      let reader = &mut request.reader;
      let protocol = reader.read_u8()?;
      let client_session_id = reader.read_u32le()?;
      let ( _, username ) = reader.read_ascii_str();
      let username = username?;
      Ok( Self { protocol, client_session_id, username } )
   }
}

#[ derive( Debug ) ]
pub struct EphemeralRequest<'a> {
   pub session_id: u32,
   pub ephemeral: &'a [ u8 ],
}

impl<'a> EphemeralRequest<'a> {
   pub fn marshall( &self, writer: &mut Writer ) {
      writer.append_u32le( REQUEST_EPHEMERAL );
      writer.append_u32le( self.session_id );
      writer.append_u16le( self.ephemeral.len() as u16 );
      writer.append_bytes( self.ephemeral );
   }

   pub fn unmarshall( request: &'a mut Request ) -> Result<Self, ()> {
      let reader = &mut request.reader;
      let session_id = reader.read_u32le()?;
      let ephemeral_length = reader.read_u16le()?;
      let ( _, ephemeral ) = reader.read_bytes( ephemeral_length as usize );
      let ephemeral = ephemeral?;
      Ok( Self { session_id, ephemeral } )
   }
}

#[ derive( Debug ) ]
pub struct ProofRequest<'a> {
   pub session_id: u32,
   pub proof: &'a [ u8 ],
}

impl<'a> ProofRequest<'a> {
   pub fn marshall( &self, writer: &mut Writer ) {
      writer.append_u32le( REQUEST_PROOF );
      writer.append_u32le( self.session_id );
      writer.append_u16le( self.proof.len() as u16 );
      writer.append_bytes( self.proof );
   }

   pub fn unmarshall( request: &'a mut Request ) -> Result<Self, ()> {
      let reader = &mut request.reader;
      let session_id = reader.read_u32le()?;
      let proof_length = reader.read_u16le()?;
      let ( _, proof ) = reader.read_bytes( proof_length as usize );
      let proof = proof?;
      Ok( Self { session_id, proof } )
   }
}
