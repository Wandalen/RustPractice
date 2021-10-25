
// callbacks are defined as template parameters
fn run< Operation, Print >( val : i32, operation : Operation, print : Print )
where
  Operation : Fn( i32 ) -> i32,
  Print : Fn( i32 )
{
  let val2 = operation( val );
  print( val2 );
}

//

fn print0( val : i32 )
{
  println!( "{}!", val );
}

//

fn main()
{

  let print = | val | println!( "{}", val );

  // pass callbacks as arguments
  let operation1 = | val | val*2;
  run( 3, operation1, print );
  // : 6

  // another set of callbacks
  let operation2 = | val | val+10;
  run( 3, operation2, print );
  // : 13

  // ordinary routine instead of anonimous
  let operation2 = | val | val+10;
  run( 3, operation2, print0 );
  // : 13!

}
