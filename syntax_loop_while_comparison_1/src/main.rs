fn main()
{

  // the only way to escape the loop is break
  // so compiler know variable `a` will be initialized
  let a;
  loop
  {
    if true
    {
      a = 13;
      break;
    }
  }
  println!( "a : {}", a );

  // despite logically clear that the only way to escape the loop is break
  // compiler fails to deduce that variable `a` will be initialized
  // assuming `while true` may be `false`
  // < let a;
  // < while true
  // < {
  // <   if true
  // <   {
  // <     a = 13;
  // <     break;
  // <   }
  // < }
  // < println!( "a : {}", a );
  // ! borrow of possibly-uninitialized variable: `a`

  // despite the face else branch does not initialize variable a
  // compiler knows that `println` is uncreachable from the else branch
  // what makes the compiler happy about
  // because the case "a is not initialized" is impossible
  let a;
  if true
  {
    a = 13;
  }
  else
  {
    loop {}
  }
  println!( "a : {}", a );

}
