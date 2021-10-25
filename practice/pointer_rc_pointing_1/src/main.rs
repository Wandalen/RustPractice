use std::rc::Rc;

fn main()
{

  let rc0 = Rc::new( vec![ 1.0, 2.0, 3.0 ] );
  let rc1 = rc0.clone();
  let rc2 = Rc::clone( &rc0 );
  let rc3 = Rc::clone( &rc2 );

  println!( "@rc0 : {:p}", rc0 );
  println!( "@rc1 : {:p}", rc1 );
  println!( "@rc2 : {:p}", rc2 );
  println!( "@rc3 : {:p}", rc3 );

  println!( "rc0.as_ptr : {:?}", Rc::as_ptr( &rc0 ) );
  println!( "rc1.as_ptr : {:?}", Rc::as_ptr( &rc1 ) );
  println!( "rc2.as_ptr : {:?}", Rc::as_ptr( &rc2 ) );
  println!( "rc3.as_ptr : {:?}", Rc::as_ptr( &rc3 ) );

  println!( "rc0.strong_count : {}", Rc::strong_count( &rc0 ) );
  println!( "rc1.strong_count : {}", Rc::strong_count( &rc1 ) );
  println!( "rc2.strong_count : {}", Rc::strong_count( &rc2 ) );
  println!( "rc3.strong_count : {}", Rc::strong_count( &rc3 ) );

}
