
fn main()
{

  let x = solution1::< i32 >();
  println!( "{}", x );
  let x = solution1::< i64 >();
  println!( "{}", x );

  let x : i32 = solution1();
  println!( "{}", x );
  let x : i64 = solution1();
  println!( "{}", x );

}

pub fn solution1< T >() -> T
where
  T: Default,
{
  T::default()
}

// ! pub fn solution2() -> impl Display
// ! {
// !   T::default()
// ! }
// can't be done with `impl Trait`
