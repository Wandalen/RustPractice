fn main()
{

  println!( "Is i32 copyable : {}", copy( 13_i32 ) );
  println!( "Is f32 copyable : {}", copy( 13_f32 ) );
  println!( "Is bool copyable : {}", copy( false ) );
  // < println!( "Is String copyable : {}", copy( "abc".to_string() ) );
  // ! the trait `Copy` is not implemented for `String`
  // String is not copyable to other routines

}

fn copy< T >( _ : T ) -> bool
where
  T : Copy,
{
  true
}