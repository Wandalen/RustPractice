
/*
declarative munching macro to
*/

//

macro_rules! munch
{
  () => {};
  ( $name : ident ; $( $tail : tt )* ) =>
  {
    {
      println!( concat!( stringify!( $name ) ) );
      munch!( $( $tail )* );
    }
  };
  ( $name : ident = $init : expr; $( $tail : tt )* ) =>
  {
    {
      // let $name = $init;
      println!( concat!( stringify!( $name ), " = ", stringify!( $init ) ) );
      munch!( $( $tail )* );
    }
  };
}

//

fn main()
{
  munch!( trace ; abc = def ; ghi ; );
}

//
