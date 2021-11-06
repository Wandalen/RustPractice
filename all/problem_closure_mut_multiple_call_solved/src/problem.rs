fn main()
{

  #[ derive( Debug ) ]
  struct Struct1
  {
    a : i32,
  }
  let mut struct1 = Struct1 { a : 1 };

  let mut step = ||
  {
    struct1.a += 1;
  };

  step();
  dbg!( &struct1 );
  step();
  dbg!( &struct1 );

}
