use nixauth::{
   requests::*,
   writer::*,
   reader::*,
};

#[ test ]
fn marshall_negotiate_request() {
   let writer = {
      let request = NegotiateRequest {
         protocol: 2,
         client_session_id: 12345,
         username: "user"
      };
      let mut writer = Writer::new();
      request.marshall( &mut writer );
      writer
   };
   let mut reader = Reader::new( writer.contents() );
   let mut request = {
      let request = Request::from_reader( &mut reader ).unwrap();
      assert!( request.header == REQUEST_NEGOTIATE );
      request
   };
   let request = NegotiateRequest::unmarshall( &mut request ).unwrap();
   assert!( request.protocol == 2 );
   assert!( request.client_session_id == 12345 );
   assert!( request.username == "user" );
}

#[ test ]
fn marshall_ephemeral_request() {
   let writer = {
      let request = EphemeralRequest {
         session_id: 12345,
         ephemeral: "abc".as_bytes(),
      };
      let mut writer = Writer::new();
      request.marshall( &mut writer );
      writer
   };
   let mut reader = Reader::new( writer.contents() );
   let mut request = {
      let request = Request::from_reader( &mut reader ).unwrap();
      assert!( request.header == REQUEST_EPHEMERAL );
      request
   };
   let request = EphemeralRequest::unmarshall( &mut request ).unwrap();
   assert!( request.session_id == 12345 );
   assert!( request.ephemeral == "abc".as_bytes() );
}

#[ test ]
fn marshall_proof_request() {
   let writer = {
      let request = ProofRequest {
         session_id: 12345,
         proof: "abc".as_bytes(),
      };
      let mut writer = Writer::new();
      request.marshall( &mut writer );
      writer
   };
   let mut reader = Reader::new( writer.contents() );
   let mut request = {
      let request = Request::from_reader( &mut reader ).unwrap();
      assert!( request.header == REQUEST_PROOF );
      request
   };
   let request = ProofRequest::unmarshall( &mut request ).unwrap();
   assert!( request.session_id == 12345 );
   assert!( request.proof == "abc".as_bytes() );
}
