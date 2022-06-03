fn main()
{
  Tuple2::< i32, f32 >( 13, 13.0 );
}

pub struct Tuple2< T1, T2 >
(
  pub T1,
  pub T2,
);
