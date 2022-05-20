pub mod server;
pub mod requests;
pub mod response;
pub mod error;
pub mod reader;
pub mod writer;
pub mod handler;
pub mod receiver;
pub mod storage;
mod session;
pub mod sqlite_storage;
pub mod log;

pub use server::Server;

fn f() {
   println!( "{}", module_path!() );
}

pub struct Registration<'storage> {
   storage: &'storage mut dyn storage::Storage,
}

impl<'storage> Registration<'storage> {
   pub fn new( storage: &'storage mut dyn storage::Storage ) -> Self {
      Self { storage }
   }

   pub fn perform( &mut self, username: &str, password: &str ) -> bool {
      let ( salt, ver ) = crsrp::create_password_verifier(
         crsrp::HashAlgorithm::Sha256,
         crsrp::Group::Bits2048, username.as_bytes(), password.as_bytes() ).unwrap();
      self.storage.add_user( username, ver.as_slice(), salt.as_slice() )
   }
}
