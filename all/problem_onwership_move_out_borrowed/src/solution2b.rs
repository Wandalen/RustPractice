use byte_slice_cast::*;
use owning_ref::*;

fn main()
{
  let mut context = context_make();
  dbg!( &context );

  dbg!( &context.dst_buffer() );
  dbg!( &context.dst_bytes() );

  context.dst_buffer()[ 1 ] = 13.;
  context.dst_bytes()[ 1 ] = 13;

  dbg!( context.dst.as_owner() );
  dbg!( &*context.dst );
}

//

fn context_make() -> Context
{
  Context::new()
}

//

#[ derive( Debug ) ]
struct Context
{
  // pub dst_buffer : Box::< [ f32 ] >,
  // pub dst_buffer_bytes : &'a [ u8 ],
  pub dst : OwningRefMut< Box::< [ f32 ] >, [ u8 ] >
}

impl Context
{
  pub fn dst_buffer( &mut self ) -> &mut Box::< [ f32 ] >
  {
    self.dst.as_owner_mut()
  }
  pub fn dst_bytes( &mut self ) -> &mut [ u8 ]
  {
    &mut *self.dst
  }
}

//

impl Context
{
  fn new() -> Context
  {
    let len : usize = 2;
    let dst_buffer : Box<[ f32 ]> = vec![ 0_f32; len ].into_boxed_slice();
    // let dst_buffer_bytes = dst_buffer.as_byte_slice();

    let dst = OwningRefMut::new( dst_buffer );
    let dst = dst.map_mut( | dst_buffer | dst_buffer.as_mut_byte_slice() );

    Context { dst }
    // Context { dst_buffer, dst_buffer_bytes }
  }
}
