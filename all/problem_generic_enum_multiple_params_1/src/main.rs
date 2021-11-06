
use std::fmt;
// use std::mem;
use std::any;
use std::rc::Rc;
// use std::any::Any;
// use std::any::TypeId;
use std::cmp::PartialEq;

fn main()
{

  let a1 = Logic::val( 'a' );
  let a2 = Logic::val( 'a' );
  println!( "a1 : {:?}", a1 );
  println!( "a2 : {:?}", a2 );
  assert_eq!( a1, a2 );

  let a1 = Logic::aref( 'a' );
  let a2 = Logic::aref( 'a' );
  println!( "a1 : {:?}", a1 );
  println!( "a2 : {:?}", a2 );
  assert_eq!( a1, a2 );

  let a1 = Logic::val( 'a' );
  let a2 = Logic::aref( 'a' );
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
pub enum Logic< ElRef = Nothing, ElCopy : Copy = Nothing >
{
  ValNode( ElCopy ),
  RefNode( Rc< ElRef > ),
  NotNode( Box<Logic< ElRef, ElCopy >> ),
}

//

impl Logic
{

  pub fn val< ElCopy : Copy >( val : ElCopy ) -> Logic< Nothing, ElCopy >
  {
    Logic::ValNode( val )
  }

  pub fn aref< ElRef >( p : ElRef ) -> Logic< ElRef, Nothing >
  {
    Logic::RefNode( Rc::new( p ) )
  }

  pub fn not< T, ElCopy : Copy, L >( l : L ) -> Logic< T, ElCopy >
  where
    L : Into<Logic< T, ElCopy >>,
  {
    Logic::NotNode( Box::new( l.into() ) )
  }

}

//

impl< ElRef, ElCopy : Copy > Clone for Logic< ElRef, ElCopy >
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

// impl<  >
// Logic< RightRef, RightCopy > : 'static
// {

// }


//

impl< LeftRef, RightRef, LeftCopy : Copy, RightCopy : Copy > PartialEq< Logic< RightRef, RightCopy > > for Logic< LeftRef, LeftCopy >
where
  LeftCopy : PartialEq,
  LeftRef : PartialEq,
  RightCopy : PartialEq,
  RightRef : PartialEq,
  // Logic< LeftRef, LeftCopy > : 'static,
  // Logic< RightRef, RightCopy > : 'static,
{
  fn eq( &self, _other : &Logic< RightRef, RightCopy > ) -> bool
  {

    return self.identical_generic( _other );

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

    false

    // if let Some( other_cast ) = ( other as &dyn Any ).downcast_ref::< Logic< LeftRef, LeftCopy > >()
    // {
    //   match *self
    //   {
    //     Logic::ValNode( e1 ) =>
    //     {
    //       match other
    //       {
    //         Logic::ValNode( e2 ) => return identical_tolerant2( e1, e2 ),
    //         _ => unreachable!(),
    //       }
    //     }
    //     _ => unreachable!(),
    //   }
    // }
    // else
    // {
    //   false
    // }

  }
}

//

// fn identical_tolerant2< T1 : Any + PartialEq, T2 : Any + PartialEq>( a : T1, b : T2 ) -> bool
// {
//   if TypeId::of::< T1 >() == TypeId::of::< T2 >()
//   {
//     let b_as_t = &b as &dyn Any;
//     &a == b_as_t.downcast_ref::< T1 >().unwrap()
//   }
//   else
//   {
//     false
//   }
// }

// trait AlsoStatic< A >
// {
//   type Static : A + 'static;
// }

//

pub trait IdenticalGeneric< B >
{
  fn identical_generic( self : &Self, _b : &B ) -> bool
  {
    return false;
  }
}

//

impl< A : PartialEq > IdenticalGeneric< A > for A
{
  fn identical_generic( self : &Self, b : &A ) -> bool
  {
    self == b
  }
}

//

// impl< A : PartialEq + 'static, B : PartialEq + 'static > IdenticalGeneric< B > for A
// {
//   fn identical_generic( self : &Self, b : &B ) -> bool
//   {
//     if any::TypeId::of::< A >() == any::TypeId::of::< B >()
//     {
//       let b_as_a = b as & dyn any::Any;
//       self == b_as_a.downcast_ref::< Self >().unwrap()
//     }
//     else
//     {
//       false
//     }
//   }
// }

//

// impl< A, B > IdenticalGeneric< B > for A
// {
//   fn identical_generic( self : &Self, b : &B ) -> bool
//   {
//     return false;
//   }
// }

//

// fn identical_generic< A, B >( a : &A, b : &B ) -> bool
// where
//   A : PartialEq< B >,
//   B : PartialEq + FromMaybe< B >,
//   // A2 : A + 'static,
//   // B2 : B + 'static,
// {
//   // if mem::discriminant( a ) == mem::discriminant( b )
//   if any::TypeId::of::< A + 'static >() == any::TypeId::of::< B + 'static >()
//   {
//     a == b
//   }
//   else
//   {
//     false
//   }
// }

//

pub trait FromMaybe< Src >
where
  Self : Sized
{
  fn from_maybe( _src : Src ) -> Option< Self >
  {
    return None;
  }
}

//

pub trait IntoMaybe< Dst > : Sized
{
  fn into_maybe( self ) -> Option< Dst >;
}

impl< Src, Dst > IntoMaybe< Dst > for Src
where
  Dst : FromMaybe< Src >,
{
  fn into_maybe( self ) -> Option< Dst >
  {
    Dst::from_maybe( self )
  }
}

//

#[cfg( test )]
mod test
{

  use super::*;

  #[test]
  fn basic()
  {

    let a_u = Box::new( 3_u32 );
    let a2_u = Box::new( 3_u32 );
    let b_u = Box::new( 13_u32 );
    let a_i = Box::new( 3_i32 );
    assert_eq!( true, a_u.identical_generic( &a_u ) );
    assert_eq!( true, a_u.identical_generic( &a2_u ) );
    assert_eq!( false, a_u.identical_generic( &b_u ) );
    assert_eq!( false, a_u.identical_generic( &a_i ) );

    #[ derive( PartialEq ) ]
    struct Struct1 { pub a : i32 }
    #[ derive( PartialEq ) ]
    struct Struct2 { pub b : i32 }
    let a_u = Struct1 { a : 3 };
    let a2_u = Struct1 { a : 3 };
    let b_u = Struct1 { a : 13 };
    let a_i = Struct2 { b : 3 };
    assert_eq!( true, a_u.identical_generic( &a_u ) );
    assert_eq!( true, a_u.identical_generic( &a2_u ) );
    assert_eq!( false, a_u.identical_generic( &b_u ) );
    assert_eq!( false, a_u.identical_generic( &a_i ) );

    struct Struct3 { pub a : i32 }
    struct Struct4 { pub b : i32 }
    let a_u = Struct3 { a : 3 };
    let a2_u = Struct3 { a : 3 };
    let b_u = Struct3 { a : 13 };
    let a_i = Struct4 { b : 3 };
    assert_eq!( true, a_u.identical_generic( &a_u ) );
    assert_eq!( true, a_u.identical_generic( &a2_u ) );
    assert_eq!( false, a_u.identical_generic( &b_u ) );
    assert_eq!( false, a_u.identical_generic( &a_i ) );

  }

}