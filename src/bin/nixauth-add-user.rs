use nixauth::sqlite_storage::SqliteStorage;
use nixauth::Registration;

use cmd_arg_iter::iter::Iter;
use cmd_arg_iter::iter::Name;

use std::io::Read;

struct Args {

}

pub fn main() {
   let args = std::env::args_os()
      .skip( 1 )
      .map( |arg| arg.into_string().unwrap() )
      .collect::<Vec<_>>();
   let mut help = false;
   let mut args = Iter::new( args );
   while let Some( option ) = args.read_option() {
      if let Name::Long( "help" ) = option {
         help = true;
      }
   }
   if help {
      usage();
      std::process::exit( 1 );
   }

   let username = args.read_value().unwrap_or_else( || {
      panic!( "missing username" );
   } );
   println!( "username: {}", username );

   let mut storage = SqliteStorage::open( "test.db" );
   let password = rpassword::prompt_password_stdout(
      format!( "Enter password for {}: ", username ).as_str() )
      .unwrap();
   println!( "Password: {}", password );
   if password.is_empty() {
      println!( "error: empty password" );
      std::process::exit( 1 );
   }

   let mut registration = Registration::new( &mut storage );
   if ! registration.perform( username, &password ) {
      println!( "error: failed to add user" );
      std::process::exit( 1 );
   }
}

fn usage() {
   println!( "Add a user to the authentication server" );
   println!();
   println!( "Usage: " );
   println!( "  nixauth-add-user [options] <username>" );
   println!();
   println!( "Options:" );
   println!( "  --help          Show help information" );
}
