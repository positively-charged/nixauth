use nixauth::{
   error::*,
   writer::*,
   reader::*,
};

#[ test ]
fn marshall_user_error() {
   let writer = {
      let error = UserError {
         error: ERR_USER_IGNORED,
         client_session_id: 12345,
      };
      let mut writer = Writer::new();
      error.marshall( &mut writer );
      writer
   };
   let mut reader = Reader::new( writer.contents() );
   assert!( reader.read_u32le().unwrap() == ERR_USER_ID );
   let error = UserError::unmarshall( &mut reader ).unwrap();
   assert!( error.error == ERR_USER_IGNORED );
   assert!( error.client_session_id == 12345 );
}

#[ test ]
fn marshall_session_error() {
   let writer = {
      let error = SessionError {
         error: ERR_SESSION_FAILURE,
         session_id: 12345,
      };
      let mut writer = Writer::new();
      error.marshall( &mut writer );
      writer
   };
   let mut reader = Reader::new( writer.contents() );
   assert!( reader.read_u32le().unwrap() == ERR_SESSION_ID );
   let error = SessionError::unmarshall( &mut reader ).unwrap();
   assert!( error.error == ERR_SESSION_FAILURE );
   assert!( error.session_id == 12345 );
}
