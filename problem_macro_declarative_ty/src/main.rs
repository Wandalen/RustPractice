
macro_rules! dup
{
  ( $T : ty ) =>
  {
    $T, $T,
  };
}

fn main()
{

  let got : ( dup!( i32 ) ) = ( 1, 1 );
  // let got : ( i32, i32, ) = ( 1, 1 );
  let exp : ( i32, i32 ) = ( 1, 1 );
  assert_eq!( got, exp );

}
