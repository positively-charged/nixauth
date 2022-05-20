use std::mem::size_of;
use std::mem::size_of_val;
use std::convert::TryInto;
use std::str;

pub struct Reader<'a> {
   pos: usize,
   content: &'a [ u8 ],
}

impl<'a> Reader<'a> {
   pub fn new( content: &[ u8 ] ) -> Reader {
      Reader { pos: 0, content }
   }

   pub fn read_u8( &mut self ) -> Result<u8, ()> {
      match self.content.get( self.pos ) {
         Some( value ) => {
            self.pos += size_of_val( value );
            Ok( *value )
         },
         None => Err( () )
      }
   }

   pub fn read_u16le( &mut self ) -> Result<u16, ()> {
      match self.content.get( self.pos .. self.pos + size_of::<u16>() ) {
         Some( slice ) => {
            let value = u16::from_le_bytes( slice.try_into().unwrap() );
            self.pos += size_of_val( &value );
            Ok( value )
         },
         None => Err( () )
      }
   }

   pub fn read_u32le( &mut self ) -> Result<u32, ()> {
      match self.content.get( self.pos .. self.pos + size_of::<u32>() ) {
         Some( slice ) => {
            let value = u32::from_le_bytes( slice.try_into().unwrap() );
            self.pos += size_of_val( &value );
            Ok( value )
         },
         None => Err( () )
      }
   }

   pub fn read_ascii_str( &mut self ) -> ( &mut Self, Result<&str, ()> ) {
      match self.content[ self.pos .. ].iter().position( |x| *x == b'\0' ) {
         Some( end ) => {
            let end = self.pos + end;
            let value = self.content.get( self.pos .. end ).unwrap();
            if ! value.is_ascii() {
               return ( self, Err( () ) );
            }
            let value = str::from_utf8( value ).unwrap();
            self.pos = end + 1;
            ( self, Ok( value ) )
         },
         None => ( self, Err( () ) ),
      }
   }

   pub fn read_bytes( &mut self, length: usize ) -> ( &mut Self,
      Result<&[ u8 ], ()> ) {
      match self.content.get( self.pos .. self.pos + length ) {
         Some( value ) => {
            self.pos += value.len();
            ( self, Ok( value ) )
         },
         None => ( self, Err( () ) ),
      }
   }
}
