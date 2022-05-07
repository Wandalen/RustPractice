//

macro_rules! primitives
{
  () =>
  {
    i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64,
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
  for_each!( generate, primitives );
}
