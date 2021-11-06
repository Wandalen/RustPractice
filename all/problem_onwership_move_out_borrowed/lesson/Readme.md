# Moving out both owned and borrowed entities out of the function and a self-referential struct.

Function returns both owned and borrowed entities and produce self-referential struct.

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