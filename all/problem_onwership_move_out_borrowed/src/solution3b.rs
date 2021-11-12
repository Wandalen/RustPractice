#![feature(generic_associated_types)]
#![allow(dead_code)]

use std::io;
use io::Write;
use internal::Context;

fn main()
{
  let mut context = context_make();

  dbg!( &context.dst_buffer() );
  dbg!( &context.dst_cursor() );

  dbg!( &context );
  println!( "dst_buffer : {:p}", context.dst_buffer() );
  println!( "dst_cursor : {:p}", &context.dst_cursor().get_ref()[ .. ] );

  let data = [ 1, 2, 3, 4, 5, 6, 7, 8, 9 ];
  context.dst_cursor().write( &data[ .. ] ).unwrap();
  dbg!( &context );
  println!( "dst_buffer : {:p}", context.dst_buffer() );
  println!( "dst_cursor : {:p}", &context.dst_cursor().get_ref()[ .. ] );

  let buffer_used_after_free : &'static mut [u8] =
  {
    let mut context = context_make();
    let data = [ 1, 2, 3, 4, 5, 6, 7, 8, 9 ];
    context.dst_cursor().write( &data[ .. ] ).unwrap();
    // context.dst_cursor().into_inner()
    dbg!( &context );
    let cursor = std::mem::take( context.dst_cursor() );
    dbg!( &context );
    let slice = cursor.into_inner();
    println!( "{:?}", slice );
    slice
  };
  println!( "{:?}", buffer_used_after_free ); /* xxx : problem! */

}

//

fn context_make< 'a >() -> Context< 'a >
{
  Context::< 'a >::new()
}

//

mod internal
{

  use std::io;
  use io::Cursor;
  use byte_slice_cast::*;

  #[ derive( Debug ) ]
  pub struct Context< 'a >
  {
    dst_buffer : Box::< [ f32 ] >,
    // dst_bytes : &'a mut [ u8 ],
    dst_cursor : std::io::Cursor< &'a mut [ u8 ] >,
  }

  impl< 'a > Context< 'a >
  {
    pub fn dst_buffer( &mut self ) -> &[ f32 ]
    {
      &self.dst_buffer[ .. ]
    }
    pub fn dst_cursor( &mut self ) -> &mut std::io::Cursor< &'a mut [ u8 ] >
    {
      &mut self.dst_cursor
    }
  }

  //

  impl< 'a > Context< 'a >
  {
    pub fn new() -> Context< 'a >
    {
      let len : usize = 2;
      let mut dst_buffer : Box<[ f32 ]> = vec![ 0_f32; len ].into_boxed_slice();

      let dst_bytes : &mut [ u8 ] = unsafe
      {
        let dst_slice = dst_buffer.as_mut_byte_slice();
        std::slice::from_raw_parts_mut( dst_slice.as_mut_ptr(), dst_slice.len() )
      };

      println!( "dst_buffer : {:p}", &dst_buffer[ .. ] );
      println!( "dst_bytes : {:p}", dst_bytes );

      let dst_cursor = Cursor::new( dst_bytes );

      println!( "dst_cursor : {:p}", &dst_cursor.get_ref()[ .. ] );

      Context { dst_buffer, dst_cursor }
    }
  }

}