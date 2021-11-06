
/*
using macro to implement trait for different primitive types
*/

fn main()
{
  println!( "i32 : {} .. {}", ( 0_i32 ).limit_low(), ( 0_i32 ).limit_high() );
}

trait MinMax
{
  fn limit_low( &self ) -> Self;
  fn limit_high( &self ) -> Self;
}

macro_rules! min_max_impl
{
  ( $Type : ty ) =>
  {
    impl $crate::MinMax for $Type
    {
      fn limit_low( &self ) -> Self
      {
        <$Type>::MIN
      }
      fn limit_high( &self ) -> Self
      {
        <$Type>::MAX
      }
    }
  }
}

min_max_impl!( i8 );
min_max_impl!( i16 );
min_max_impl!( i32 );
min_max_impl!( i64 );
min_max_impl!( i128 );

min_max_impl!( u8 );
min_max_impl!( u16 );
min_max_impl!( u32 );
min_max_impl!( u64 );
min_max_impl!( u128 );

min_max_impl!( f32 );
min_max_impl!( f64 );
