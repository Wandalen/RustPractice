use std::rc::Rc;

use super::problem::*;

pub fn main()
{

  let context = Rc::new( Context { a : 13 } );
  let mut engine = Engine::new();

  let context_clone_1 = Rc::clone( &context );
  engine.on_event1( Box::new( move ||
  {
    println!( "on_event1 : {}", context_clone_1.a );
  }));

  let context_clone_2 = Rc::clone( &context );
  engine.on_event2( Box::new( move ||
  {
    println!( "on_event2 : {}", context_clone_2.a );
  }));

  engine.listen();

}
