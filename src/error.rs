use crate::reader::Reader;
use crate::writer::Writer;

pub const ERR_USER_ID: u32 = 0xD003CAFF;
pub const ERR_SESSION_ID: u32 = 0xD003CAEE;

#[ derive( Debug ) ]
pub enum ErrorResponse {
   User( UserError ),
   Session( SessionError ),
}

impl ErrorResponse {
   pub fn show( &self ) {
      match self {
         Self::User( err ) => println!( "{:?}", err ),
         Self::Session( err ) => println!( "{:?}", err ),
      }
   }

   pub fn marshall( &self, writer: &mut Writer ) {
      match self {
         Self::User( err ) => err.marshall( writer ),
         Self::Session( err ) => err.marshall( writer ),
      }
   }
}

pub const ERR_USER_BUSY: u8 = 0;
pub const ERR_USER_NOT_FOUND: u8 = 1;
pub const ERR_USER_UNSUPPORTED_PROTOCOL: u8 = 2;
pub const ERR_USER_IGNORED: u8 = 3;

#[ derive( Debug ) ]
pub struct UserError {
   pub error: u8,
   pub client_session_id: u32,
}

impl UserError {
   pub fn marshall( &self, writer: &mut Writer ) {
      writer.append_u32le( ERR_USER_ID );
      writer.append_u8( self.error );
      writer.append_u32le( self.client_session_id );
   }

   pub fn unmarshall( reader: &mut Reader ) -> Result<Self, ()> {
      let error = reader.read_u8()?;
      let client_session_id = reader.read_u32le()?;
      Ok( Self { error, client_session_id } )
   }
}

pub const ERR_SESSION_BUSY: u8 = 0;
pub const ERR_SESSION_NOT_FOUND: u8 = 1;
pub const ERR_SESSION_VERIFIER_UNSAFE: u8 = 2;
pub const ERR_SESSION_FAILURE: u8 = 3;

#[ derive( Debug ) ]
pub struct SessionError {
   pub error: u8,
   pub session_id: u32,
}

impl SessionError {
   pub fn marshall( &self, writer: &mut Writer ) {
      writer.append_u32le( ERR_SESSION_ID );
      writer.append_u8( self.error );
      writer.append_u32le( self.session_id );
   }

   pub fn unmarshall( reader: &mut Reader ) -> Result<Self, ()> {
      let error = reader.read_u8()?;
      let session_id = reader.read_u32le()?;
      Ok( Self { error, session_id } )
   }
}
