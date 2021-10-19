fn main()
{
  let a = 10 * { let b = 1; let c = 2; b + c };
  println!( "a : {}", a );
  // : a : 30
}
