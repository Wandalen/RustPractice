fn main()
{

  let a = ( 13, );
  println!( "{:?}", &a as *const ( i32, ) );
  println!( "{:?}", &a.0 as *const i32 );
  println!( "{:p}", &a );
  // prints the same address

}
