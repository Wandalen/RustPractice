#![feature(type_name_of_val)]

use core::fmt::Display;

fn main()
{
  let x = f1();
  // ! x += 1;
  // !> binary assignment operation `+=` cannot be applied to type `impl std::fmt::Display`
  println!( "{}", x );

}

//

pub fn f1() -> impl Display
{
  13_i32
}
