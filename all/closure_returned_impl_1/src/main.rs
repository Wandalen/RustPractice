fn main()
{

  let double = closure_for( "double" );
  let triple = closure_for( "triple" );
  let quadruple = closure_for( "quadruple" );

  let number = 5;
  println!( "Double of {} is {}", number, double( number ) );
  println!( "Triple of {} is {}", number, triple( number ) );
  println!( "Quadruple of {} is {}", number, quadruple( number ) );

}

//

fn closure_for( what : &str ) -> impl Fn( i32 ) -> i32
{
  match what
  {
    "double" => | number | { number * 2 },
    "triple" => | number | { number * 3 },
    "quadruple" => quadruple,
    _ => | _number | { -1 },
  }
}

//

fn quadruple( src : i32 ) -> i32
{
  src * 4
}