#![allow(non_snake_case)]
// #![recursion_limit="256"]

use std::rc::Rc;
use std::fmt;
// use std::mem;

use std::any::Any;
use std::cmp::PartialEq;
use std::any::TypeId;

//

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

// #[derive( Debug, PartialEq, Eq, Hash )]
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

  // pub fn identical_operation< R, V : Copy, L >( &self, e : &Logic< R, V > ) -> bool
  // {
  //   mem::discriminant( self ) == mem::discriminant( e )
  // }

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

    // match *self
    // {
    //   Logic::ValNode( val1 ) =>
    //   {
    //     match other
    //     {
    //       Logic::ValNode( val2 ) => return val1 == val2,
    //       Logic::RefNode( ref val2 ) => return val1 == val2,
    //       Logic::NotNode( ref val2 ) => return false,
    //     }
    //   }
    //   Logic::RefNode( ref val1 ) =>
    //   {
    //     match other
    //     {
    //       Logic::ValNode( val2 ) => return val1 == val2,
    //       Logic::RefNode( ref val2 ) => return val1 == val2,
    //       Logic::NotNode( ref val2 ) => return false,
    //     }
    //   }
    //   Logic::NotNode( ref val1 ) =>
    //   {
    //     match other
    //     {
    //       Logic::ValNode( val2 ) => return false,
    //       Logic::RefNode( ref val2 ) => return false,
    //       Logic::NotNode( ref val2 ) => return val1 == val2,
    //     }
    //   }
    // }

  }
}

//

// fn identical_tolerant1( a : impl PartialEq + 'static, b : impl PartialEq + 'static ) -> bool
// {
//   if let Some( b_cast ) = ( &b as &dyn Any ).downcast_ref()
//   {
//     &a == b_cast
//   }
//   else
//   {
//     false
//   }
// }

//

fn identical_tolerant2< T : Any + PartialEq, Q : Any + PartialEq>( a : T, b : Q ) -> bool
{
  if TypeId::of::<T>() == TypeId::of::<Q>()
  {
    let b_as_t = &b as &dyn Any;
    &a == b_as_t.downcast_ref::<T>().unwrap()
  }
  else
  {
    false
  }
}

//

// fn eq< A : PartialEq + 'static, B : PartialEq + 'static >( a : A, b : B ) -> bool
// {
//   if std::any::TypeId::of::<A>() != std::any::TypeId::of::<B>()
//   {
//     return false
//   }
//   else
//   {
//     unsafe
//     {
//       return std::mem::transmute::< A, B >( a ) == b;
//     }
//   }
// }

//

// fn eq<A:'static, B:'static>(_:A, _:B)->bool
// {
//   std::any::TypeId::of::<A>() == std::any::TypeId::of::<B>()
// }
