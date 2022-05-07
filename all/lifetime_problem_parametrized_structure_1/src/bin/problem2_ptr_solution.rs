/* Is that proper solution to use pointers instead fo references? */

use core::ops::Index;
use std::collections::HashMap;

fn main()
{
}

pub type HandlerDyn< 'dy, Event > = dyn HandlerInterface
<
  Event,
> + 'dy;

pub trait HandlerInterface< Event >
where
  Event : EventInterface,
{}

pub trait EventInterface
{}

pub trait ReactorInterface< Event >
where
  Event : EventInterface,
{
  type ReceiversMap : for< 'key > Index< &'key usize, Output = Box< HandlerDyn< 'static, Event > > >;
  fn handlers( &self ) -> &Self::ReceiversMap;
  fn handlers_mut( &mut self ) -> &mut Self::ReceiversMap;
}

pub struct Event1
{
  a : *mut u32,
}

impl Event1
{

  pub fn a< 'a >( &'a self ) -> &'a u32
  {
    unsafe
    {
      std::mem::transmute::< _, &'a _ >( self.a )
    }
  }

  pub fn a_mut< 'a >( &'a mut self ) -> &'a mut u32
  {
    unsafe
    {
      std::mem::transmute::< _, &'a mut _ >( self.a )
    }
  }

}

impl< 'a > EventInterface for Event1
{

}

pub type HandlerOfEventDyn = dyn HandlerInterface
<
  Event1,
> + 'static;

pub struct Reactor
{
  handlers : HashMap< usize, Box< HandlerOfEventDyn > >,
}

impl ReactorInterface< Event1 > for Reactor
{
  type ReceiversMap = HashMap< usize, Box< HandlerOfEventDyn > >;
  fn handlers( &self ) -> &Self::ReceiversMap
  {
    &self.handlers
  }
  fn handlers_mut( &mut self ) -> &mut Self::ReceiversMap
  {
    &mut self.handlers
  }
}

struct Struct1< 'a >
{
  b : &'a i32,
}

pub struct Event
{
  a : *mut Struct1< ? >,
}
