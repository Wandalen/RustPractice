use core::fmt::Display;

fn main()
{
  f1( 13_i32 );
  f1( 13_i64 );
}

//

pub fn f1( src : impl Display )
{
  // ! src += 1;
  // !> binary assignment operation `+=` cannot be applied to type `impl std::fmt::Display`
  println!( "{}", src );
}
