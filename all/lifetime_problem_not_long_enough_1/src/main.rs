// #![ feature( trait_alias ) ]

fn main()
{

  let handler = Box::new( Handler {} );
  let owner = Owner{ handler };
  let e = Event{};
  owner.send( e );

}

//

trait EventInterface
{
}

type DynEventInterface< 'a > = dyn EventInterface + 'a;

//

#[ derive( Debug ) ]
struct Event
{
}

impl EventInterface for Event
{
}

//

trait HandlerInterface
{
  fn recieve( &self, e : &DynEventInterface );
}

//

struct Handler
{
}

//

impl HandlerInterface for Handler
{
  fn recieve( &self, _e : &DynEventInterface )
  {
    println!( "recieved" );
  }
}

//

struct Owner
{
  pub handler : Box< ( dyn HandlerInterface + 'static ) >,
}

impl Owner
{
  fn send< Event : EventInterface >( &self, e : Event )
  // where
  //   Event : 'static,
  {
    self.handler.recieve( &e );
  }
}
