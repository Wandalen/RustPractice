
use std::rc::Rc;
use std::fmt;
use std::any::Any;
use std::cmp::PartialEq;
use std::any::TypeId;

fn main()
{

  let a1 = Logic::Val( 'a' );
  let a2 = Logic::Val( 'a' );
  println!( "a1 : {:?}", a1 );
  println!( "a2 : {:?}", a2 );
  assert_eq!( a1, a2 );

  let a1 = Logic::Ref( 'a' );
  let a2 = Logic::Ref( 'a' );
  println!( "a1 : {:?}", a1 );
  println!( "a2 : {:?}", a2 );
  assert_eq!( a1, a2 );

  let a1 = Logic::Val( 'a' );
  let a2 = Logic::Ref( 'a' );
  println!( "a1 : {:?}", a1 );
  println!( "a2 : {:?}", a2 );
  assert_eq!( a1, a2 );

}

//

#[derive( Debug, PartialEq, Eq, Hash, Clone, Copy )]
pub struct Nothing
{
}

//

impl fmt::Display for Nothing
{
  fn fmt( &self, f : &mut fmt::Formatter ) -> fmt::Result
  {
    write!( f, "{}", "{- Nothing -}" )
  }
}

//

#[derive( Debug, Hash )]
pub enum Logic< R = Nothing, V : Copy = Nothing >
{
  ValNode( V ),
  RefNode( Rc< R > ),
  NotNode( Box<Logic< R, V >> ),
}

//

impl Logic
{

  pub fn Val< V : Copy >( val : V ) -> Logic< Nothing, V >
  {
    Logic::ValNode( val )
  }

  pub fn Ref< R >( p : R ) -> Logic< R, Nothing >
  {
    Logic::RefNode( Rc::new( p ) )
  }

  pub fn Not< T, V : Copy, L >( l : L ) -> Logic< T, V >
  where
    L : Into<Logic< T, V >>,
  {
    Logic::NotNode( Box::new( l.into() ) )
  }

}

//

impl< R, V : Copy > Clone for Logic< R, V >
{
  fn clone( &self ) -> Self
  {
    match *self
    {
      Logic::ValNode( t ) => Logic::ValNode( t ),
      Logic::RefNode( ref p ) => Logic::RefNode( p.clone() ),
      Logic::NotNode( ref e ) => Logic::NotNode( e.clone() ),
    }
  }
}

//

impl< LeftRef, RightRef, LeftCopy : Copy, RightCopy : Copy > PartialEq< Logic< RightRef, RightCopy > > for Logic< LeftRef, LeftCopy >
where
  LeftCopy : PartialEq,
  LeftRef : PartialEq,
  RightCopy : PartialEq,
  RightRef : PartialEq,
  Logic< LeftRef, LeftCopy > : 'static,
  Logic< RightRef, RightCopy > : 'static,
{
  fn eq( &self, other : &Logic< RightRef, RightCopy > ) -> bool
  {

    if let Some( other_cast ) = ( other as &dyn Any ).downcast_ref::< Logic< LeftRef, LeftCopy > >()
    {
      match *self
      {
        Logic::ValNode( e1 ) =>
        {
          match other
          {
            Logic::ValNode( e2 ) => return identical_tolerant2( e1, e2 ),
            _ => unreachable!(),
          }
        }
        _ => unreachable!(),
      }
    }
    else
    {
      false
    }
  }
}

//

fn identical_tolerant2< T1 : Any + PartialEq, T2 : Any + PartialEq>( a : T1, b : T2 ) -> bool
{
  if TypeId::of::< T1 >() == TypeId::of::< T2 >()
  {
    let b_as_t = &b as &dyn Any;
    &a == b_as_t.downcast_ref::< T1 >().unwrap()
  }
  else
  {
    false
  }
}
