
pub struct Context
{
  pub a : i32,
}

//

pub type OnEvent1 = dyn FnMut();
pub type OnEvent2 = dyn FnMut();

//

pub struct Engine
{
  pub f_on_event1 : Box< OnEvent1 >,
  pub f_on_event2 : Box< OnEvent2 >,
}

impl Engine
{

  pub fn new() -> Engine
  {
    let f_on_event1 = || {};
    let f_on_event2 = || {};
    Engine { f_on_event1 : Box::new( f_on_event1 ), f_on_event2 : Box::new( f_on_event2 ) }
  }

  pub fn on_event1( &mut self, r : Box< OnEvent1 > )
  {
    self.f_on_event1 = r;
  }

  pub fn on_event2( &mut self, r : Box< OnEvent2 > )
  {
    self.f_on_event2 = r;
  }

  pub fn listen( &mut self )
  {

    (self.f_on_event1)();
    (self.f_on_event2)();

  }

}
