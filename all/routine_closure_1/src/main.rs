fn main()
{
  let mut c = 0;
  // anonimous routine should be mut to change values outside of its scope
  let mut inc = | i |
  {
    c += 1;
    i + c
  };
  // closure with memory

  let a = 13;
  let b = inc( a );
  let c = inc( a );

  println!( "a : {}", a );
  println!( "b : {}", b );
  println!( "c : {}", c );

}