pub struct ActualLog {

}

impl ActualLog {
   pub fn write( &mut self, line: &str ) {
      println!( "{}", line );
   }
}


pub struct Log {
   log: std::cell::RefCell<ActualLog>,
}

impl Log {
   pub fn new() -> Self {
      let log = ActualLog {};
      Self {
         log: std::cell::RefCell::new( log ),
      }
   }

   pub fn write<T: AsRef<str>>( &self, line: &T ) {
      self.log.borrow_mut().write( line.as_ref() );
   }
}
