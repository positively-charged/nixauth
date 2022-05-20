use nixauth::Server;
use nixauth::sqlite_storage::SqliteStorage;
use cmd_arg_iter::iter::{ Iter, Name };

struct Options {
   database_path: String,
   server_port: i32,
   help: bool,
}

fn main() {
   let options = read_options();
   if options.database_path.is_empty() {
      println!( "error: missing database-path argument" );
      std::process::exit( 1 );
   }
   let mut storage = SqliteStorage::open( options.database_path.as_str() );
   //println!( "{}", options.server_port );
   let server = Server::new( &mut storage, options.server_port );
   server.run();
}

fn read_options() -> Options {
   let args = std::env::args_os()
      .skip( 1 )
      .map( |arg| arg.into_string().unwrap() )
      .collect::<Vec<_>>();
   let mut args = Iter::new( args );
   let mut options = Options { database_path: "".to_string(), server_port: 16666,
      help: false };
   while let Some( option ) = args.read_option() {
      match option {
         Name::Short( "d" ) |
         Name::Long( "database-path" ) => options.database_path =
            args.read_value().unwrap().to_string(),
         Name::Short( "p" ) |
         Name::Long( "port" ) => options.server_port =
            args.read_value().unwrap().parse::<i32>().unwrap(),
         _ => {
            println!( "error: invalid option" );
            std::process::exit( 1 );
         }
      }
   }
   options
}
