use nixauth::{
   response::*,
   writer::*,
   reader::*,
};

#[ test ]
fn marshall_negotiate_response() {
   let writer = {
      let response = NegotiateResponse {
         client_session_id: 12345,
         session_id: 54321,
         salt: "abc".as_bytes().to_vec(),
         username: "user".to_string()
      };
      let mut writer = Writer::new();
      response.marshall( &mut writer );
      writer
   };
   let mut reader = Reader::new( writer.contents() );
   assert!( reader.read_u32le().unwrap() == RESPONSE_NEGOTIATE );
   let response = NegotiateResponse::unmarshall( &mut reader ).unwrap();
   assert!( response.client_session_id == 12345 );
   assert!( response.session_id == 54321 );
   assert!( response.salt == "abc".as_bytes() );
   assert!( response.username == "user" );
}

#[ test ]
fn marshall_ephemeral_response() {
   let writer = {
      let response = EphemeralResponse {
         session_id: 12345,
         ephemeral: "abc".as_bytes().to_vec(),
      };
      let mut writer = Writer::new();
      response.marshall( &mut writer );
      writer
   };
   let mut reader = Reader::new( writer.contents() );
   assert!( reader.read_u32le().unwrap() == RESPONSE_EPHEMERAL );
   let response = EphemeralResponse::unmarshall( &mut reader ).unwrap();
   assert!( response.session_id == 12345 );
   assert!( response.ephemeral == "abc".as_bytes() );
}

#[ test ]
fn marshall_proof_response() {
   let writer = {
      let response = ProofResponse {
         session_id: 12345,
         proof: "abc".as_bytes().to_vec(),
      };
      let mut writer = Writer::new();
      response.marshall( &mut writer );
      writer
   };
   let mut reader = Reader::new( writer.contents() );
   assert!( reader.read_u32le().unwrap() == RESPONSE_PROOF );
   let response = ProofResponse::unmarshall( &mut reader ).unwrap();
   assert!( response.session_id == 12345 );
   assert!( response.proof == "abc".as_bytes() );
}
