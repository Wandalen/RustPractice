#![allow(dead_code)]

fn main()
{

  let struct1 = Struct1
  {
    a : OnDrop { name : "a".into() },
    b : OnDrop { name : "b".into() },
    c : OnDrop { name : "c".into() },
  };

  drop( struct1 );

}

//

pub struct OnDrop
{
  name : String,
}

impl Drop for OnDrop
{
  fn drop( &mut self )
  {
    println!( "Dropping {}", self.name );
  }
}

//

pub struct Struct1
{
  c : OnDrop,
  b : OnDrop,
  a : OnDrop,
}
