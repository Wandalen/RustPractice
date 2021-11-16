use core::fmt::Display;

fn main()
{
  let x = not_a_problem( 1 );
  println!( "{}", x );
}

//

pub fn not_a_problem( a : i32 ) -> impl Display
{
  if a > 0
  {
    return 13_i32
  }
  else
  {
    return -13_i32
  }
}

//

// !pub fn corner_case( a : i32 ) -> impl Display
// !{
// !  if a > 0
// !  {
// !    return 13_i32
// !  }
// !  else
// !  {
// !    return -13_i64
// !  }
// ! }
// !> to return `impl Trait`, all returned values must be of the same type
