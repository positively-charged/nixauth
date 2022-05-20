 fn main() {
   println!( "cargo:rustc-link-lib=csrp" );
   println!( "cargo:rustc-link-lib=ssl" );
   println!( "cargo:rustc-link-lib=crypto" );
   println!( "cargo:rustc-link-search=native=./lib/csrp" );
   println!( "{:?}", std::env::current_dir().unwrap() );
   std::process::exit(0);
 }
