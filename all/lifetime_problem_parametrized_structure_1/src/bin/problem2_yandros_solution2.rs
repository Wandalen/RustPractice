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
  type Handler : ?Sized + 'static + HandlerInterface< Event >;
  type ReceiversMap : for< 'key > Index< &'key usize, Output = Box< Self::Handler > >;

  fn handlers( &self ) -> &Self::ReceiversMap;
  fn handlers_mut( &mut self ) -> &mut Self::ReceiversMap;
}

pub struct Event1< 'a >
{
  a : &'a u32,
}

impl< 'a > EventInterface for Event1< 'a >
{
}

pub type HandlerOfEventDyn = dyn for< 'a > HandlerInterface
<
  Event1< 'a >,
> + 'static;

pub struct Reactor
{
  handlers : HashMap< usize, Box< HandlerOfEventDyn > >,
}

impl< 'a > ReactorInterface< Event1< 'a > > for Reactor
{

  type Handler = HandlerOfEventDyn;
  type ReceiversMap = HashMap< usize, Box< Self::Handler > >;

  fn handlers( &self ) -> &Self::ReceiversMap
  {
    &self.handlers
  }
  fn handlers_mut( &mut self ) -> &mut Self::ReceiversMap
  {
    &mut self.handlers
  }
}
