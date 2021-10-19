use paste::paste;

/*
macro of concatenation to synth an identifier using module::paste
*/

macro_rules! hi
{
  ( $id : tt ) =>
  {
    paste!( [< hi_ $id >] )( stringify!( $id ) );
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
