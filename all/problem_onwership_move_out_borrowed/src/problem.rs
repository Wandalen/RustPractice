#![ feature( new_uninit ) ]

use byte_slice_cast::*;
use std::io;

fn main()
{

  let context = context_make();
  dbg!( &context );

}

//

fn context_make< 'a >() -> Context< std::io::Cursor< &'a mut [ u8 ] > >
{
  Context::< std::io::Cursor< &mut [ u8 ] > >::new()
}

//

#[ derive( Debug ) ]
struct Context< W >
where
  W : std::io::Write + std::io::Seek,
{
  pub dst_cursor : W,
  pub dst_buffer : Box::< [ f32 ] >,
}

//

impl< W > Context< W >
where
  W : std::io::Write + std::io::Seek,
{
  fn new< 'a >() -> Context< std::io::Cursor< &'a mut [ u8 ] > >
  {
    let len : usize = 1024;
    let mut dst_buffer = vec![ 0_f32; len ].into_boxed_slice();
    let dst_buffer_bytes = dst_buffer.as_mut_byte_slice();
    let dst_cursor = io::Cursor::new( dst_buffer_bytes );
    Context { dst_cursor, dst_buffer };
  }
}
