
/*

article :: Finding Closure in Rust :: http://huonw.github.io/blog/2015/05/finding-closure-in-rust/

*/

fn main()
{

  stock();
  custom();

}

//

// < fn move_problem( x: i32 ) -> Box< Fn(i32) -> i32 >
// < {
// <   Box::new( |y| x + y )
// < }
// ! closure may outlive the current function, but it borrows `x`, which is owned by the current function

//

fn stock()
{
  let option = Some( 13 );

  let x = 3;
  // explicit types:
  let new: Option<i32> = option.map( |val: i32| -> i32 { val + x } );
  println!("{:?}", new); // Some(5)

  let y = 10;
  // inferred:
  let new2 = option.map(|val| val * y);
  println!("{:?}", new2); // Some(20)

}

//

fn custom()
{
  let option = Some( 13 );

  let x = 3;
  let new: Option<i32> = map( option, ClosureAdder { x } );
  println!( "{:?}", new ); // Some(5)

  let y = 10;
  let new2 = map( option, ClosureMultiplier { y } );
  println!( "{:?}", new2 ); // Some(20)

}

//

trait Closure<Input>
{
  type Output;
  fn closure( self, input : Input ) -> Self::Output;
}

//

fn map< X, Y, T >( option : Option< X >, closure : T ) -> Option< Y >
  where T: Closure< X, Output = Y >
{
    match option
    {
        Some(x) => Some( closure.closure( x ) ),
        None => None,
    }
}

// replacement for |val| val + x

struct ClosureAdder { x : i32 }
impl Closure<i32> for ClosureAdder
{
  type Output = i32;

  // ignoring the `fn ... self`, this looks similar to |val| val + x
  fn closure( self, val : i32 ) -> i32
  {
    val + self.x
  }
}

// replacement for |val| val * y

struct ClosureMultiplier { y: i32 }
impl Closure<i32> for ClosureMultiplier
{
  type Output = i32;

  // looks similar to |val| val * y
  fn closure(self, val: i32) -> i32
  {
    val * self.y
  }

}

//
