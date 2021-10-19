fn main()
{

  // < let a = 13_i32;
  // < let b : i64 = a;
  // < println!( "b : {}", b );
  // ! expected `i64`, found `i32`

  let a : i32 = 13;
  let b : i64 = a as i64;
  println!( "b : {}", b );
  // : b : 13

  let a : i64 = 13;
  let b : i32 = a as i32;
  println!( "b : {}", b );
  // : b : 13

  let a : i32 = 13;
  let b : i64 = a.into();
  println!( "b : {}", b );
  // : b : 13
  // method into converts the value into another type without lose of data

  // < let a : i64 = 13;
  // < let b : i32 = a.into();
  // < println!( "b : {}", b );
  // ! the trait `From<i64>` is not implemented for `i32`
  // if not possible to convert into another type without loss of data then compile-time error thrown

  // < let a : isize = 13;
  // < let b : i64 = a;
  // < println!( "b : {}", b );
  // ! expected `i32`, found `isize`
  // despite the fact types isize and i64 semantically the same conversion should be done explicitly

  let a : isize = 13;
  let b : i32 = a as i32;
  println!( "b : {}", b );
  // : b : 13

  let a : i32 = 13;
  let b : f32 = a as f32;
  println!( "b : {}", b );
  // : b : 13

}
