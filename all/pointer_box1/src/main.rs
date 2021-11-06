fn main()
{

  let a = 1;
  let b = &a;
  let c = Box::new( a );
  println!( "{:?} == {:?} : {:?}", b, c, *b == *c );

  // *b and *c is not address but value
  let b1 = 1;
  let b2 = &b1;
  let c1 = 1;
  let c2 = Box::new( c1 );
  println!( "{:?} == {:?} : {:?}", b2, c2, *b2 == *c2 );

  // alternatives to get pointers
  let a_ptr = &a as *const i32;
  let b_ptr = b as *const i32;
  let c_ptr = &c as *const Box<i32>;
  println!( "a : {:?} {:p}", a, &a );
  println!( "b : {:?} {:p}", b, b );
  println!( "c : {:?} {:p}", c, c );
  println!( "&c : {:?} {:p}", &c, &c );
  println!( "a_ptr : {:?} {:p}", a_ptr, a_ptr );
  println!( "b_ptr : {:?} {:p}", b_ptr, b_ptr );
  println!( "c_ptr : {:?} {:p}", c_ptr, c_ptr );

}
