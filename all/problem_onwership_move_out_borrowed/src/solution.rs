#![ feature( new_uninit ) ]

// use owning_ref::*;
use byte_slice_cast::*;
// use std::io;

fn main()
{

  // let x : Box< [ f32 ] > = vec![ 0_f32; 3 ].into_boxed_slice();
  // dbg!( &x );

  // let x : Box< &[ f32 ] > = Box::new( &[ 1., 2., 3. ][ .. ] );
  // dbg!( &x );

  let context = context_make();
  dbg!( &context );

}

//

// fn context_make< 'a >() -> Context< 'a, std::io::Cursor< &'a mut [ u8 ] > >
fn context_make< 'a >() -> Context< 'a >
// fn context_make<>() -> Context<>
{
  // Context::new()
  Context::< 'a >::new()
  // Context::< std::io::Cursor< &mut [ u8 ] > >::new()
}

//

#[ derive( Debug ) ]
struct Context< 'a >
// struct Context< 'a, W >
// where
  // W : std::io::Write + std::io::Seek,
{
  pub dst_buffer : Box::< [ f32 ] >,
  pub dst_buffer_bytes : &'a [ u8 ],
  // pub dst_cursor : W,
  // dst : OwningRef< Box::< [ f32 ] >, W >,
}

//   let or = OwningRef::new( v );
//   let or = or.map( | v | &v[ 1..3 ] );
//   or

//

// impl< 'a, W > Context< 'a, W >
// where
//   W : std::io::Write + std::io::Seek,
impl< 'a > Context< 'a >
{
  fn new<>() -> Context< 'a >
  {
    let len : usize = 13;
    let dst_buffer : Box<[ f32 ]> = vec![ 0_f32; len ].into_boxed_slice();
    let dst_buffer_bytes = dst_buffer.as_byte_slice();
    // let dst_cursor = io::Cursor::new( dst_buffer_bytes );
    Context { dst_buffer, dst_buffer_bytes }
    // Context { dst_cursor : &dst_cursor, dst_buffer : &dst_buffer }
    // let dst = OwningRef::new( dst_buffer );
    // let dst = dst.map( | dst_buffer | &&io::Cursor::new( dst_buffer.as_mut_byte_slice() ) );
    // Context { dst }
  }
}

//

// fn return_owned_and_referenced() -> OwningRef< Vec< u8 >, [ u8 ] >
// {
//   let v = vec![ 1, 2, 3, 4 ];
//   let or = OwningRef::new( v );
//   let or = or.map( | v | &v[ 1..3 ] );
//   or
// }
