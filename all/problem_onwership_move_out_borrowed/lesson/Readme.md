# Moving out both owned and borrowed entities out of the function and a self-referential struct.

Function returns both owned and borrowed entities and produce self-referential struct. How to implement that?

```rust
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

```

[Code](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=ca788629975f932031c2e61970a3f39f)

How to be?

Simple soultion to use module `owning-ref`.

```rust
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
```

[Code](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=5af9201858b32f962b2c52c9b9af9fc0)

But what if dependant field is not reference. Lets say `Cursor` borrow bytes buffer. Also, is it possible to solve the problem without `owning_ref`?
Yes!

```rust
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
```

[Code](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=a3007ab0cc31fae6b99a25acf697a680)

Line `std::slice::from_raw_parts_mut( dst_slice.as_mut_ptr(), dst_slice.len() )` breaks relation between dst_buffer and dst_bytes and associated lifetime of `dst_cursor` with the `Context`.

