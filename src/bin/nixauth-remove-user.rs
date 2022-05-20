use nixauth::sqlite_storage::SqliteStorage;
use nixauth::storage::Storage;
use cmd_arg_iter::iter::Iter;
use cmd_arg_iter::iter::Name;

struct Options {
   database_path: String,
   server_port: i32,
   help: bool,
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

   let mut storage = SqliteStorage::open( "test.db" );
   if ! <SqliteStorage as Storage>::remove_user( &mut storage, username ) {
      println!( "error: failed to remove user: {}", username );
      std::process::exit( 1 );
   }
}

fn usage() {
   println!( "Remove a user from the authentication server" );
   println!();
   println!( "Usage: " );
   println!( "  nixauth-remove-user [options] <username>" );
   println!();
   println!( "Options:" );
   println!( "  --help          Show help information" );
}
