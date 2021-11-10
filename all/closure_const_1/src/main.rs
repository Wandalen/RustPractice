#![feature(const_trait_impl)]
#![feature(const_fn_trait_bound)]

fn main()
{
  let x = || String::from( "main" );
  println!( "{}", x() );
  println!( "{}", act1( zero ) );
  println!( "{}", act1( x ) );
}

//

fn zero() -> String
{
  String::from( "zero" )
}

//

const fn act1< F : ~ const FnOnce() -> String >( f : F ) -> String
{
  f()
}