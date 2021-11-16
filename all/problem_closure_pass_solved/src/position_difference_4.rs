#![feature(type_name_of_val)]

fn main()
{
  let x : internal::Type1 = internal::f1();
  // ! x += 1;
  // !> binary assignment operation `+=` cannot be applied to type `impl std::fmt::Display`
  println!( "{} : {:?}", std::any::type_name_of_val( &x ), x );
}

//

mod internal
{
  use core::fmt::Debug;

  #[allow( dead_code )]
  #[derive( Debug )]
  pub struct Type1
  {
    a : i32,
  }

  pub trait Trait1 {}

  impl Trait1 for Type1 {}

  // pub fn f1() -> impl Trait1 + Debug
  // {
  //   Type1 { a : 13 }
  // }

  pub fn f1< T : Trait1 + Debug >() -> T
  where
    Type1 : Trait1 + Debug
  {
    Type1 { a : 13 }
  }

}
