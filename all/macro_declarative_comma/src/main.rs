#![feature(trace_macros)]

fn f1( x : i32 ) -> i32
{
  x * 10
}

//

macro_rules! val
{

  () => {};

  ( $v : expr ) =>
  {
    f1( $v )
  };

  ( $( $rest : expr ),+ ) =>
  {(
    $( val!( $rest ) ),+,
  )};

}

//

fn main()
{

  let a = val!( 1 );
  dbg!( a );

  let x = ( { 1 }, { 2 } );
  dbg!( x );

  // !expected one of `)`, `,`, `.`, `?`, or an operator, found reserved identifier `$crate`
  trace_macros!( true );
  let b = ( val!( 1, 2 ) );
  trace_macros!( false );
  dbg!( b );

  trace_macros!( true );
  let b = ( val!( 1, 2, 3 ) );
  trace_macros!( false );
  dbg!( b );

}
