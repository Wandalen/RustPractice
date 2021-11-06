use std::rc::Rc;

use super::problem::*;

macro_rules! with_clones
{
  ( $($clone_me : ident),* ; $body:block) =>
  {
    {
      $( let $clone_me = Rc::clone( &$clone_me ); )*
      Box::new( move ||
      {
        $body
      })
    }
  }
}

//

pub fn main()
{

  let context = Rc::new( Context { a : 13 } );
  let mut engine = Engine::new();

  // engine.on_event1( Box::new( move ||
  engine.on_event1( with_clones!( context;
  {
    println!( "on_event1 : {}", context.a );
  }));

  // engine.on_event2( Box::new( move ||
  engine.on_event2( with_clones!( context;
  {
    println!( "on_event2 : {}", context.a );
  }));

  engine.listen();

}
