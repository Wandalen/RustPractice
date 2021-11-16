use core::fmt::Display;

fn main()
{
  f1::< i32, i32 >( 13, 14 );
  f2::< i32, i32 >( 13, 14 );
  // ! f3::< i32, i32 >( 13, 14 );
  // <! explicit generic argument not allowed

  f1( 13, 14 );
  f2( 13, 14 );
  f3( 13, 14 );
}

//

fn f1< A : Display + PartialEq, B : Display >( a : A, b : B )
{
  println!( "{} {} {}", a, b, a == a );
}

//

fn f2< A, B >( a : A, b : B )
where
  A : Display + PartialEq,
  B : Display,
{
  println!( "{} {} {}", a, b, a == a );
}

//

fn f3( a : impl Display + PartialEq, b : impl Display )
{
  println!( "{} {} {}", a, b, a == a );
}
