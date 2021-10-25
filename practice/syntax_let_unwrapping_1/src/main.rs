use std::result::Result;
use std::error::Error;
// unwrapping is spread known as destructing, but the name does not reflect meaning

fn main() 
{

  // unwrapping struct
  struct Struct1 { field1 : i32 }
  let struct1 = Struct1 { field1 : 13 };
  let Struct1 { field1 } = struct1;
  println!( "field1 : {}", field1 ); 
  // < field1 : 13
  let Struct1 { field1 : x } = struct1;
  println!( "x : {}", x );
  // < x : 13

  // unwrapping single-element enum
  enum Wrap { Val( i32 ) }
  let wrapped1 = Wrap::Val( 13 );
  let Wrap::Val( val1 ) = wrapped1;
  println!( "val1 : {}", val1 ); 
  // < val1 : 13

  // unwrapping multiple-elements enum
  let result1 : Result< i32, &dyn Error > = Result::Ok( 13 );
  match result1
  {
    Ok( val ) => println!( "result : {}", val ),
    Err( err ) => println!( "result : {:?}", err ),
  }
  // < result : 13

  // unwrapping several-elements enum reduced to single-element
  // ! enum Void {}
  // ! let result2 : Result< i32, Void > = Ok( 0 );  
  // ! let Ok( num ) = result2;
  // ! println!( "num : {}", num );
  // Err doesn't exist anymore, so Ok is actually irrefutable.
  // but unsupported in Rust2018
  // xxx : try nightly build

}
