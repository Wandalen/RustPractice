fn main()
{

  /* from i32 to Struct */
  {
    let src = Struct1 { a : 13 };
    let got = i32::from2( src );
    let exp = 13;
    assert_eq!( got, exp );
  }

  /* from &i32 to Struct */
  {
    let src = Struct1 { a : 13 };
    // let got = i32::from2( &src );
    // let got : i32 = ( &&&src ).into2();
    // let got : i32 = ( &&&src ).into3();
    let got : i32 = ( &&&src )._into();
    let exp = 13;
    assert_eq!( got, exp );
  }

}

pub trait From2< T > : Sized
{
  fn from2( _ : T ) -> Self;
}

pub trait Into2< T > : Sized
where
  T : From2< Self > + Sized,
{
  /// Performs the conversion.
  fn into2( self ) -> T
  {
    T::from2( self )
  }
}

impl< Target, Original > Into2< Target > for Original
where
  Target : crate::From2< Original >,
{
}

// pub trait FromInto< T > : From2< T > + Into2< T >
// {
//   fn into3< Other : From2< T > + Into2< T > >( self ) -> Other
//   where
//     Self : Sized + Copy,
//   {
//     Other::from2( self.into2() )
//   }
// }
//
// impl< T, Target > FromInto< T >
// for Target
// where
//   Target : From2< T > + Into2< T >,
//   Self : Sized + Copy,
// {}

pub trait AsNumber
{
  fn make( src : i32 ) -> Self;
  fn as_number( self ) -> i32;

  fn _into< Other :  AsNumber >( self ) -> Other
  where
    Self : Sized + Copy,
  {
    // Other::from( self.into() )
    Other::make( self.as_number() )
  }
}

impl AsNumber for i32
{
  fn make( src : i32 ) -> Self
  {
    src
  }
  fn as_number( self ) -> i32
  {
    self
  }
}

impl AsNumber for Struct1
{
  fn make( src : i32 ) -> Self
  {
    Struct1 { a : src }
  }
  fn as_number( self ) -> i32
  {
    self.a
  }
}

impl< Original, Target > From2< Original >
for Target
where
  Original : AsNumber,
  Target : AsNumber,
{
  fn from2( original : Original ) -> Self
  {
    Self::make( original.as_number() )
  }
}

#[ derive( PartialEq, Debug, Clone, Copy ) ]
struct Struct1
{
  a : i32,
}
