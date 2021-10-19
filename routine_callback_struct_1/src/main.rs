
//

struct Runner< Operation, Print >
where
  Operation : Fn( i32 ) -> i32,
  Print : Fn( i32 )
{
  operation : Operation,
  print : Print,
}

//

impl< Operation, Print > Runner< Operation, Print >
where
  Operation : Fn( i32 ) -> i32,
  Print : Fn( i32 )
{
  fn run( &self, val : i32 )
  {
    let val2 = (self.operation)( val );
    (self.print)( val2 );
  }
}

//

fn main()
{

  let print = | val | println!( "{}", val );
  let operation = | val | val*2;
  let runner = Runner { operation : operation, print };
  runner.run( 3 );
  // : 6

}
