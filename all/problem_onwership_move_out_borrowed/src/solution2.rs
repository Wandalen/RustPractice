use byte_slice_cast::*;
use owning_ref::*;

fn main()
{
  let context = context_make();
  dbg!( &context );
  dbg!( context.dst.as_owner() );
  dbg!( &*context.dst );
}

//

fn context_make<>() -> Context<>
{
  Context::<>::new()
}

//

#[ derive( Debug ) ]
struct Context<>
{
  // pub dst_buffer : Box::< [ f32 ] >,
  // pub dst_buffer_bytes : &'a [ u8 ],
  pub dst : OwningRef< Box::< [ f32 ] >, [ u8 ] >
}

//

impl<> Context<>
{
  fn new<>() -> Context<>
  {
    let len : usize = 2;
    let dst_buffer : Box<[ f32 ]> = vec![ 0_f32; len ].into_boxed_slice();
    // let dst_buffer_bytes = dst_buffer.as_byte_slice();

    let dst = OwningRef::new( dst_buffer );
    let dst = dst.map( | dst_buffer | dst_buffer.as_byte_slice() );

    Context { dst }
    // Context { dst_buffer, dst_buffer_bytes }
  }
}
