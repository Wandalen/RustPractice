fn main()
{

  let e = Event{};
  e.f1();
  ( &e as &dyn EventInterface ).f2();

  // ! ( &e as &dyn EventInterface ).f1();
  // < multiple applicable items in scope

  ( &e as &dyn EventInterface ).f3();

  // < f1 EventInterface
  // < f2 dyn EventInterface
  // < f3 EventInterface

}

//

trait EventInterface
{
  fn f1( &self )
  {
    println!( "f1 EventInterface" );
  }
  fn f3( &self )
  {
    println!( "f3 EventInterface" );
  }
}

impl dyn EventInterface
{
  fn f1( &self )
  {
    println!( "f1 dyn EventInterface" );
  }
  fn f2( &self )
  {
    println!( "f2 dyn EventInterface" );
  }
}

//

#[ derive( Debug ) ]
struct Event
{
}

impl EventInterface for Event
{
}
