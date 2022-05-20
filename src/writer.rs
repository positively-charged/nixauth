

pub struct Writer {
   buffer: Vec<u8>,
}

impl Writer {
   pub fn new() -> Self {
      Writer { buffer: Vec::new() }
   }

   pub fn append_u8( &mut self, value: u8 ) {
      self.buffer.push( value );
   }

   pub fn append_u16le( &mut self, value: u16 ) {
      self.buffer.extend( value.to_le_bytes() );
   }

   pub fn append_u32le( &mut self, value: u32 ) {
      self.buffer.extend( value.to_le_bytes() );
   }

   pub fn append_bytes( &mut self, value: &[ u8 ] ) {
      self.buffer.extend( value );
   }

   pub fn contents( &self ) -> &[ u8 ] {
      self.buffer.as_slice()
   }
}
