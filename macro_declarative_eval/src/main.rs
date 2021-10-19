
/*
declarative macro muncher evaluating expressions
*/

macro_rules! calc
{
  () =>
  {
  };
  ( eval $eval : expr ) =>
  {{
    let result = $eval;
    println!( "{} = {}", stringify!( $eval ), result );
    result
  }};
  ( eval $eval : expr; $( eval $eval2 : expr );+ $( ; )? ) =>
  {{
    calc!( eval $eval );
    calc!( $( eval $eval2 );+ )
  }};
}

//

fn main()
{
  calc!
  {
    eval 13 + 15
  };
}

//

#[ test ]
fn single()
{

  let result = calc!
  {
    eval 13 + 15
  };
  assert_eq!( result, 28 );

}

//

#[ test ]
fn several()
{

  let result = calc!
  {
    eval 13 + 15;
    eval 13 * 5;
    eval 6 / 3;
  };
  assert_eq!( result, 2 );

}
