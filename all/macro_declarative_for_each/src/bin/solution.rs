//

macro_rules! apply
{
  ( $Callback:ident, $( $Arg:tt ),* ) =>
  {
    $( $Callback!( $Arg ); )*
  }
}

//

macro_rules! for_each_number
{
  ( $Callback:ident ) =>
  {
    apply!( $Callback, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64 )
  }
}

//

macro_rules! generate
{
  ( $Type : ident ) =>
  {
    let x : $Type; /* some code */
  }
}

//

fn main()
{
  for_each_number!( generate );
}
