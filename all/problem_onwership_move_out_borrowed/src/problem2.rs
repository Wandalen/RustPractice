use byte_slice_cast::*;

fn main()
{
  let context = context_make();
  dbg!( &context );
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
  pub dst_buffer : Box::< [ f32 ] >,
  pub dst_buffer_bytes : &'a [ u8 ],
}

//

impl< 'a > Context< 'a >
{
  fn new<>() -> Context< 'a >
  {
    let len : usize = 13;
    let dst_buffer : Box<[ f32 ]> = vec![ 0_f32; len ].into_boxed_slice();
    let dst_buffer_bytes = dst_buffer.as_byte_slice();
    Context { dst_buffer, dst_buffer_bytes }
  }
}
