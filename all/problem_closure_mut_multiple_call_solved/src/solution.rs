fn main()
{

  #[ derive( Debug ) ]
  struct Struct1
  {
    a : i32,
  }
  let mut struct1 = Struct1 { a : 1 };

  impl Struct1
  {
    fn step( &mut self )
    {
      self.a += 1;
    }
  }

  struct1.step();
  dbg!( &struct1 );
  struct1.step();
  dbg!( &struct1 );

}
