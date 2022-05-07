macro_rules! identity
{
  (
    $( $Src : tt )*
  )
  =>
  {
    $( $Src )*
  };
}

fn main()
{
  dbg!( "a" );
  identity!( dbg!( "a" ); );
}
