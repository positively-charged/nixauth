
//mod server;

#[ test ]
fn test_server() {

/*
   // user registration.
   let private_key = srp_private_key::<Sha256>( USERNAME.as_bytes(),
      PASSWORD.as_bytes(), SALT.as_bytes() );

   // client sends pub a.
   let mut a = [ 0u8; 8 ];
   rand::thread_rng().fill_bytes( &mut a );
   let client = SrpClient::<Sha256>::new( &a, &G_2048 );
   let pub_a = client.get_a_pub();
   let v = client.get_password_verifier( &private_key );

   // server sends pub b.
   let mut b = [ 0u8; 8 ];
   rand::thread_rng().fill_bytes( &mut b );
   let user = UserRecord { username: USERNAME.as_bytes(), salt: SALT.as_bytes(), verifier: &v };
   let server = SrpServer::<Sha256>::new( &user, &pub_a, &b, &G_2048 ).unwrap();
   let pub_b = server.get_b_pub();

   // client creates proof.
   let verifier = client.process_reply( &private_key, &pub_b ).unwrap();
   let user_proof = verifier.get_proof();

   // server verifies client proof.
   let server_proof = server.verify( &user_proof ).unwrap();

   // client verifies server proof.
   let key = verifier.verify_server( &server_proof ).unwrap();

   for x in private_key {
      print!( "{:x}", x );
   }
   println!( "" );
   return;
   */
   /*
   let mut request = Request::new();
   println!( "{}", request.content.capacity() );

   let mut writer = Writer::new();
   writer.append_u32le( REQUEST_NEGOTIATE );
   writer.append_u8( 2 );
   writer.append_u32le( 123 );
   writer.append_bytes( b"positron\0" );
   request.content = writer.contents().to_vec();

   let server = Server::new();
   let response = server.handle_request( &mut request );
   response.unwrap();*/
}
