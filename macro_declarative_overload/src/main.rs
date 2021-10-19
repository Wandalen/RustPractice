
/*
declarative macro overloading
*/

macro_rules! logic
{
  ( $l : tt and $r : expr ) =>
  {
    println!( "{} and {} = {}", $l, $r, $l && $r );
  };
  ( $l : tt or $r : expr ) =>
  {
    println!( "{} or {} = {}", $l, $r, $l || $r );
  };
}

//

fn main()
{
  logic!( true and false );
  logic!( true or false );
}
