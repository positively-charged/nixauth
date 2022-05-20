use nixauth::handler::Handler;
use nixauth::requests::*;
use nixauth::error::*;
use nixauth::sqlite_storage::SqliteStorage;
use nixauth::session::SessionList;
use nixauth::log::Log;

use crsrp::*;

/*
use rand::RngCore;
use srp::{ client::{ SrpClient },
   groups::{ G_2048 } };
use sha2::Sha256;*/

#[ test ]
fn all() {
   verify_user();
   invalid_protocol();
   user_not_found();
}

#[ test ]
#[ ignore ]
fn verify_user() {
   let log = Log::new();
   let mut sessions = SessionList::new( &log, 10, 5 );
   let mut storage = get_storage();
   let mut handler = Handler::new( &mut storage, &mut sessions );

/*
   let response = handler.handle_negotiate( &NegotiateRequest {
      protocol: 2,
      client_session_id: 12345,
      username: "test"
   } ).unwrap_or_else( |response| {
      response.show();
      panic!();
   } ); */

   use nixauth::writer::Writer;
   use nixauth::reader::Reader;
   use nixauth::receiver::Receiver;
   use nixauth::response::*;
   use nixauth::requests::*;

   let mut receiver = Receiver::new( &mut handler );

   let mut writer = Writer::new();
   NegotiateRequest {
         protocol: 2,
         client_session_id: 12345,
         username: "test"
   }.marshall( &mut writer );
   let mut content = Reader::new( writer.contents() );
   let mut res = Writer::new();
   if ! receiver.handle_request( &mut content, &mut res ) {
      panic!();
   }

   let mut content = Reader::new( res.contents() );
   assert!( content.read_u32le().unwrap() == RESPONSE_NEGOTIATE );
   //let mut content = Request::from_reader( &mut content ).unwrap();
   let response = NegotiateResponse::unmarshall( &mut content ).unwrap();

   assert!( response.client_session_id == 12345 );
   //assert!( response.salt == SALT.as_bytes() );
   println!( "{}", response.client_session_id );
   println!( "{}", response.session_id );
   println!( "{:?}", response.salt );
   println!( "{}", response.username );
   let username = response.username;
   let salt = response.salt;

   // Client sends pub a.
   let session_id = response.session_id;
   //let mut a = [ 0u8; 32 ];
   //rand::thread_rng().fill_bytes( &mut a );
   //let client = SrpClient::<Sha256>::new( &a, &G_2048 );
   //let mut pub_a = client.get_a_pub();

   let password = "12345";
   let mut user = user::User::new( crsrp::HashAlgorithm::Sha256,
      crsrp::Group::Bits2048, username.as_bytes(), password.as_bytes(),
      &salt );
   let public_key = user.create_public_key();

/*
   let ( user, pub_a ) = unsafe {
      let user = srp_user_new( HashAlgorithm::SHA256, NGType::Ng2048,
         format!( "{}\0", username ).as_bytes().as_ptr() as *const libc::c_char,
         format!( "{}\0", password ).as_bytes().as_ptr(),
         password.len() as i32, std::ptr::null(),
         std::ptr::null(), 1 );
      let mut username_alias: *const libc::c_char = std::ptr::null();
      let mut bytes_A: *const libc::c_uchar = std::ptr::null();
      let mut len_A: i32 = 0;
      srp_user_start_authentication( user, &mut username_alias,
         &mut bytes_A,
         &mut len_A );
      let pub_a = ( *std::ptr::slice_from_raw_parts( bytes_A,
         len_A as usize ) ).to_vec();
      println!( "user: {:?}", pub_a );
      ( user, pub_a )
   };

   let response = handler.handle_ephemeral( &EphemeralRequest {
      session_id: response.session_id,
      ephemeral: pub_a.as_slice(),
   } ).unwrap_or_else( |response| {
      response.show();
      panic!();
   } );
   */

   let mut writer = Writer::new();
   EphemeralRequest {
      session_id: response.session_id,
      ephemeral: public_key.as_slice(),
   }.marshall( &mut writer );
   let mut content = Reader::new( writer.contents() );
   let mut res = Writer::new();
   if ! receiver.handle_request( &mut content, &mut res ) {
    panic!();
   }
   let mut content = Reader::new( res.contents() );
   assert!( content.read_u32le().unwrap() == RESPONSE_EPHEMERAL );
   //let mut content = Request::from_reader( &mut content ).unwrap();
   let response = EphemeralResponse::unmarshall( &mut content ).unwrap();

         println!( "{:?}", response );
   assert!( response.session_id == session_id );
   /*
   let private_key = srp::client::srp_private_key::<Sha256>(
      username.as_bytes(), PASSWORD.as_bytes(), salt.as_slice() );
   let verifier = client.process_reply( &private_key,
      &response.ephemeral ).unwrap();
     */
   //let mut proof = verifier.get_proof();
   let proof = user.process_challenge( &response.ephemeral ).unwrap();

/*
   let proof = unsafe {
      let mut bytes_M: *const libc::c_uchar = std::ptr::null();
      let mut len_M: i32 = 0;
      srp_user_process_challenge( user,
         salt.as_slice().as_ptr(), salt.as_slice().len() as i32,
         response.ephemeral.as_ptr(),
         response.ephemeral.len() as i32,
         &mut bytes_M, &mut len_M );
         if ( bytes_M == std::ptr::null() ) {
            panic!("User SRP-6a safety check violation!\n");
         }
      ( *std::ptr::slice_from_raw_parts( bytes_M,
         len_M as usize ) ).to_vec()
   };

   let response = handler.handle_proof( &ProofRequest {
      session_id: response.session_id,
      proof: &proof,
   } ).unwrap_or_else( |response| {
      response.show();
      panic!();
   } );*/

   let mut writer = Writer::new();
   ProofRequest {
      session_id: response.session_id,
      proof: &proof,
   }.marshall( &mut writer );
   let mut content = Reader::new( writer.contents() );
   let mut res = Writer::new();
   if ! receiver.handle_request( &mut content, &mut res ) {
    panic!();
   }
   let mut content = Reader::new( res.contents() );
   assert!( content.read_u32le().unwrap() == RESPONSE_PROOF );
   //let mut content = Request::from_reader( &mut content ).unwrap();
   let response = ProofResponse::unmarshall( &mut content ).unwrap();

   // client verifies server proof.
   assert!( response.session_id == session_id );
   let session = user.verify_session( &response.proof ).unwrap();
   /*
   unsafe {
      srp_user_verify_session( user, response.proof.as_slice().as_ptr() );
      assert!( srp_user_is_authenticated( user ) == 1 );
      let mut key_length: i32 = 0;
      let key = srp_user_get_session_key( user, &mut key_length );
      println!( "user session key: {:?}", &*std::ptr::slice_from_raw_parts(
         key, key_length as usize ) );
   }*/

   //verifier.verify_server( &response.proof ).unwrap();

   unsafe {
      //srp_user_delete( user );
   }
}

pub static USERNAME: &str = "test";
pub static PASSWORD: &str = "12345";
//pub static SALT: &str = "xxx";

fn get_storage() -> SqliteStorage {
   // let mut storage = SqliteStorage::new_in_memory();

      /*
   let private_key = srp::client::srp_private_key::<Sha256>(
      USERNAME.as_bytes(),
      PASSWORD.as_bytes(),
      SALT.as_bytes()
   );
   let a = [ 0u8; 8 ];
   let client = SrpClient::<Sha256>::new( &a, &G_2048 );
   let verifier = client.get_password_verifier( &private_key );
   storage.add_user(
      USERNAME,
      verifier.as_slice(),
      SALT.as_bytes() );*/

   let storage = SqliteStorage::open( "test.db" );
   /*let storage = SqliteStorage::new( "test.db" );
   let ( salt, verifier ) = unsafe {
      let mut bytes_s: *const libc::c_uchar = std::ptr::null_mut();
      let mut len_s: i32 = 0;
      let mut bytes_v: *const libc::c_uchar = std::ptr::null_mut();
      let mut len_v: i32 = 0;
      srp_create_salted_verification_key(
         HashAlgorithm::SHA256, NGType::Ng2048,
         format!( "{}\0", USERNAME ).as_bytes().as_ptr() as *const libc::c_char,
         format!( "{}\0", PASSWORD ).as_bytes().as_ptr(),
         PASSWORD.len() as i32,
         &mut bytes_s, &mut len_s,
         &mut bytes_v, &mut len_v,
         std::ptr::null(), std::ptr::null() );
      if ( bytes_s == std::ptr::null_mut() ||bytes_v == std::ptr::null_mut() ) {
         panic!("failed to create verification key!");
      }
      ( ( *std::ptr::slice_from_raw_parts( bytes_s,
         len_s as usize ) ).to_vec(),
         ( *std::ptr::slice_from_raw_parts( bytes_v,
            len_v as usize ) ).to_vec() )
   };
   storage.add_user(
      USERNAME,
      verifier.as_slice(),
      salt.as_slice() );*/
   storage
}

#[ test ]
#[ ignore ]
fn invalid_protocol() {
   let log = Log::new();
   let mut sessions = SessionList::new( &log, 10, 5 );
   let mut storage = get_storage();
   let mut handler = Handler::new( &mut storage, &mut sessions );

   handler.handle_negotiate( &NegotiateRequest {
      protocol: 1,
      client_session_id: 12345,
      username: "abcde"
   } ).map_err( |response| {
      match response {
         ErrorResponse::User( err ) => {
            assert!( err.client_session_id == 12345 );
            assert!( err.error == ERR_USER_UNSUPPORTED_PROTOCOL );
         }
         _ => panic!(),
      }
   } ).map( |_| panic!() );
}

#[ test ]
#[ ignore ]
fn user_not_found() {

   let log = Log::new();
   let mut sessions = SessionList::new( &log, 10, 5 );
   let mut storage = get_storage();
   let mut handler = Handler::new( &mut storage, &mut sessions );

   handler.handle_negotiate( &NegotiateRequest {
      protocol: 2,
      client_session_id: 12345,
      username: "not-found"
   } ).map_err( |response| {
      match response {
         ErrorResponse::User( err ) => {
            assert!( err.client_session_id == 12345 );
            assert!( err.error == ERR_USER_NOT_FOUND );
         }
         _ => panic!(),
      }
   } ).map( |_| panic!() );
}
