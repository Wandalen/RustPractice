#![feature(concat_idents)]

/*
macro of concatenation to synth an identifier using feature::concat_idents
*/

macro_rules! hi
{
  ( $id : tt ) =>
  {
    concat_idents!( hi_, $id )( stringify!( $id ) );
  }
}

//

fn main()
{
  hi!( there );
}

//

fn hi_there( src : &str )
{
  println!( "Hi {}!", src );
}
