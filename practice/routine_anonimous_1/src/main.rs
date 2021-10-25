fn main()
{
  let inc = | i | i + 1;
  // type is optional

  let a = 13;
  let b = inc( a );
  // same as ordinary routine

  println!( "a : {}", a );
  println!( "b : {}", b );

}
