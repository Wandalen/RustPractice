/* Event without references */

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
}

pub struct Event1
{
}

impl EventInterface for Event1
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
}
