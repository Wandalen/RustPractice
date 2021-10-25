fn functor() -> Box< dyn Fn( i32 ) -> i32 >
{
  // < let routine = | val : i32 | val + 1;
  // < Box::new( move routine )
  // : expected one of `async`, `|`, or `||`, found `routine`
  Box::new( move | val : i32 | val + 1 )
}

//

fn main()
{
  // functor generates routine
  let r = functor();
  let a = 1;
  let b = r( a );
  println!( "b : {}", b );
}
