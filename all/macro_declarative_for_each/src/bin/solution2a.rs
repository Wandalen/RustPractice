//

// macro_rules! apply
// {
//   ( $Callback:ident @PREFIX( $( $Prefix : tt )* ) @ARGS( $( $Arg:tt ),* ) ) =>
//   {
//     $( $Callback!( @PREFIX( $( $Prefix )* ) @ARGS( $Arg ) ); )* /* ! meta-variable `Prefix` repeats 0 times, but `Arg` repeats 12 times */
//     // $( $Callback!( @PREFIX() @ARGS( $Arg ) ); )* /* <- works if prefix is not paseed */
//   }
// }

macro_rules! apply
{
  (
    $Callback:ident
    @PREFIX( $( $Prefix : tt )* )
    @ARGS( $( $Arg:tt ),* )
  ) =>
  {
    $Callback!
    (
      @PREFIX( $( $Prefix )* )
      @ARGS( $( $Arg )* )
    );
  }
}

//

macro_rules! for_each_number
{
  ( $Callback:ident $( $Prefix : tt )* ) =>
  {
    apply!( $Callback @PREFIX() @ARGS( i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64 ) )
  }
}

//

macro_rules! generate
{
  ( @PREFIX( $( $Prefix : tt )* ) @ARGS( $( $Args : tt )* ) ) =>
  {
    println!( "{}", stringify!( $( $Args )* ) ); /* some code */
    $( $Prefix )*
  }
}

//

fn main()
{
  for_each_number!( generate println!( "hello!" ) );
}
