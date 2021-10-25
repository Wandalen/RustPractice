fn main()
{

  // while is also an expression
  // while always returns unit
  let mut i = 0;
  let a = while i < 5
  {
    i += 1;
  };
  println!( "a : {:?}", a );
  // : a : ()

  // while always returns unit
  // < let mut i = 0;
  // < let a = while i < 5
  // < {
  // <   i += 1;
  // <   13
  // < };
  // < println!( "a : {:?}", a );
  // ! expected `()`, found integer

}