fn main()
{

  // if like expression
  let a = if 10 > 5
  {
    13
  }
  else
  {
    3
  };
  println!( "a : {:?}", a );
  // : a : 13

  // trailing semicolon returns unit, otherwise last statement returned
  // <  let a = if 10 > 5
  // <  {
  // <    13
  // <  }
  // <  else
  // <  {
  // <    3;
  // < };
  // < println!( "a : {:?}", a );
  // ! `if` and `else` have incompatible types

  // can only return unit
  // < let a = if 10 > 5
  // < {
  // <   13
  // < };
  // < println!( "a : {:?}", a );
  // ! note: `if` expressions without `else` evaluate to `()`

  // returning unit is ok
  let a = if 10 > 5
  {
    13;
  };
  println!( "a : {:?}", a );
  // : a : ()
  // trailing ";" returns unit

}