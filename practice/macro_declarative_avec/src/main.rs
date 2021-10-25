
/*
using macro to implement vec! macro
*/

//

#[ macro_export ]
macro_rules! avec
{
  () =>
  {
    Vec::new()
  };
  ( $el : expr ) =>
  {{
    let mut result = Vec::with_capacity( 1 );
    result.push( $el );
    result
  }};
  ( $( $els : expr ),* $(,)? ) =>
  {{
    let mut result = Vec::with_capacity( $crate::_avec!( @COUNT ; $( $els ),* ) );
    $( result.push( $els ); )*
    result
  }};
  ( $el : expr; $len : expr ) =>
  {{
    let mut result = Vec::with_capacity( $len );
    result.resize( $len, $el );
    result
  }};
}

#[ macro_export ]
#[ doc( hidden ) ]
macro_rules! _avec
{
  ( @COUNT ; $( $el : expr ),* ) =>
  {
    // 13
    // [ $( $el ),* ].len()
    <[()]>::len( &[ $( $crate::_avec!( @AS_UNIT ; $el ) ),* ] )
  };
  ( @AS_UNIT ; $el : expr ) =>
  {
    ()
  };
}

//

fn main()
{
  let a : Vec< u32 > = avec!();
  println!( "a : {:?}", a );
  let b = avec!( 13 );
  println!( "b : {:?}", b );
}

//

#[ test ]
fn empty()
{
  let src : Vec< u32 > = avec!();
  assert!( src.is_empty() );
}

//

#[ test ]
fn single()
{

  let src = avec!( 13 );
  assert_eq!( src.len(), 1 );
  assert_eq!( src.capacity(), 1 );
  assert_eq!( src[ 0 ], 13 );

  let src = avec![ 13 ];
  assert_eq!( src.len(), 1 );
  assert_eq!( src.capacity(), 1 );
  assert_eq!( src[ 0 ], 13 );

  let src = avec!{ 13 };
  assert_eq!( src.len(), 1 );
  assert_eq!( src.capacity(), 1 );
  assert_eq!( src[ 0 ], 13 );

  #[ allow( dead_code ) ]
  #[ derive( Clone, Debug, PartialEq ) ]
  struct A { val : i32 }
  let a = A { val : 13 };
  let src = avec!( a );
  assert_eq!( src.len(), 1 );
  assert_eq!( src.capacity(), 1 );

}

//

#[ test ]
fn several()
{

  let src = avec!( 13, 14 );
  assert_eq!( src.len(), 2 );
  assert_eq!( src.capacity(), 2 );
  assert_eq!( src[ 0 ], 13 );
  assert_eq!( src[ 1 ], 14 );

  let src = avec!( 13, 14, );
  assert_eq!( src.len(), 2 );
  assert_eq!( src.capacity(), 2 );
  assert_eq!( src[ 0 ], 13 );
  assert_eq!( src[ 1 ], 14 );

  #[ allow( dead_code ) ]
  #[ derive( Clone, Debug, PartialEq ) ]
  struct A { val : i32 }
  let a = A { val : 1 };
  let b = A { val : 2 };
  let src = avec!( a, b );
  assert_eq!( src.len(), 2 );
  assert_eq!( src.capacity(), 2 );
  assert_eq!( src[ 0 ], A { val : 1 } );
  assert_eq!( src[ 1 ], A { val : 2 } );

}

//

#[ test ]
fn with_size()
{
  let src = avec!( 13 ; 3 );
  assert_eq!( src.len(), 3 );
  assert_eq!( src.capacity(), 3 );
  assert_eq!( src[ 0 ], 13 );
  assert_eq!( src[ 1 ], 13 );
  assert_eq!( src[ 2 ], 13 );

  #[ allow( dead_code ) ]
  #[ derive( Clone, Debug, PartialEq ) ]
  struct A { val : i32 }
  let a = A { val : 13 };
  let src = avec!( a ; 3 );
  assert_eq!( src.len(), 3 );
  assert_eq!( src.capacity(), 3 );

}
