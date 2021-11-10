#![feature(generic_associated_types)]

use byte_slice_cast::*;
// use owning_ref::*;
use std::io;
use io::Cursor;

fn main()
{
  let context = context_make();
  dbg!( &context );

  // dbg!( &context.dst_buffer() );
  // dbg!( &context.dst_bytes() );

  // context.dst_buffer()[ 1 ] = 13.;
  // context.dst_bytes()[ 1 ] = 13;

  // dbg!( context.dst.as_owner() );
  // dbg!( &*context.dst );
}

//

fn context_make< 'a >() -> Context< 'a >
{
  Context::< 'a >::new()
}

//

#[ derive( Debug ) ]
struct Context< 'a >
{
  dst_buffer : Box::< [ f32 ] >,
  pub dst_cursor : std::io::Cursor< &'a mut [ u8 ] >,
}

impl< 'a > Context< 'a >
{
}

//

impl< 'a > Context< 'a >
{
  fn new() -> Context< 'a >
  {
    let len : usize = 2;
    let mut dst_buffer : Box<[ f32 ]> = vec![ 0_f32; len ].into_boxed_slice();
    let dst_cursor = Cursor::new( dst_buffer.as_mut_byte_slice() );
    Context { dst_buffer, dst_cursor }
  }
}
