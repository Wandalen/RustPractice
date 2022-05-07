
fn main()
{
}

//

struct Field1< 'b >
{
  b : &'b i32,
}

//

struct Event1< 'a, 'b >
{
  field1 : &'a mut Field1< 'b >,
}

//

impl< 'a, 'b > EventInterface for Event1< 'a, 'b >
{
}

//

trait EventInterface
{
}

//

trait EventReceiverInterface< Event >
where
  Event : EventInterface + ?Sized,
{
  type Event : EventInterface + ?Sized;
  fn receive< 'slf, 'ev >( &'slf self, e : &'ev mut Self::Event )
  where
    'slf : 'ev,
  ;
}

//

struct Receiver
{
}

//

impl< 'a, 'b > EventReceiverInterface< Event1< 'a, 'b, > >
for Receiver
{
  type Event = Event1< 'a, 'b, >;
  fn receive< 'slf, 'ev >( &'slf self, e : &'ev mut Self::Event )
  where
    'slf : 'ev,
    // 'slf : 'b, /* how to specify something like this here? */
  {
    external1( self, &mut e.field1 );
  }
}

// function external1 is external thing and should not be changed

fn external1< 'slf, 'a, 'b >
(
  slf : &'slf Receiver,
  field1 : &'a mut Field1< 'b >,
)
where
  // 'slf : 'a,
  'slf : 'b,
{
  /* some code here */
}
