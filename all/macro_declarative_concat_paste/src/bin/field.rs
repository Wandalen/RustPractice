use paste::paste;

macro_rules! macro1
{
  ( $postfix : tt ) =>
  {
    // Struct1::$postfix();
    // Struct1::concat_idents!( prefix, $postfix )(); /* parametrization of function names is not possible? */
    paste!( Struct1::[< prefix_ $postfix >]() );
  }
}

//

fn main()
{
  // Struct1::prefix_f1();
  // Struct1::prefix_f2();
  // macro1!( prefix_f1 );
  // macro1!( prefix_f2 );
  macro1!( f1 );
  macro1!( f2 );
}

pub struct Struct1 {}

impl Struct1
{

  pub fn prefix_f1()
  {
    println!( "prefix_f1" );
  }

  pub fn prefix_f2()
  {
    println!( "prefix_f2" );
  }

}
