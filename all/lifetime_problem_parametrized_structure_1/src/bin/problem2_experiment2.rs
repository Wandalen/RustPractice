/* Mut cause problem : implementation of `ReactorInterface` is not general enough */

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

pub struct Event1< 'a >
{
  a : &'a u32,
}

impl< 'a > EventInterface for Event1< 'a >
{
}

// pub type HandlerOfEventDyn = dyn for< 'a > HandlerInterface
// <
//   Event1< 'a >,
// > + 'static;

pub type HandlerOfEventDynWithLifetime< 'a > = dyn HandlerInterface
<
  Event1< 'a >,
> + 'static;

// pub struct Reactor
// {
//   handlers : HashMap< usize, Box< HandlerOfEventDyn > >,
// }

pub struct Reactor< 'a >
{
  handlers : HashMap< usize, Box< HandlerOfEventDynWithLifetime< 'a  > > >,
}

impl< 'a > ReactorInterface< Event1< 'a > > for Reactor< 'a >
{
  type ReceiversMap = HashMap< usize, Box< HandlerOfEventDynWithLifetime< 'a > > >;
  fn handlers( &self ) -> &Self::ReceiversMap
  {
    &self.handlers
  }
  fn handlers_mut( &mut self ) -> &mut Self::ReceiversMap
  {
    &mut self.handlers
  }
}
