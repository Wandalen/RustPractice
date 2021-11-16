use core::fmt::Display;

fn main()
{
  let x = problem::< i32 >();
  println!( "{}", x );
  let x = problem::< i64 >();
  println!( "{}", x );
}

//

pub fn problem< T >() -> T
where
  T : Display,
{
  let r = 13;
  r
}
