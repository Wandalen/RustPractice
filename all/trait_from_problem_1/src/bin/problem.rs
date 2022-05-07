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
    let got = i32::from2( &src );
    let exp = 13;
    assert_eq!( got, exp );
  }

}

pub trait From2< T > : Sized
{
  fn from2( _ : T ) -> Self;
}

// // ! error[E0119]: conflicting implementations of trait `From2<&_>`
// impl< Target > From2< &Target >
// for Target
// {
//   fn from2( original : &Target ) -> Self
//   {
//     < Self as From2< Target > >::from2( *original )
//   }
// }

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

#[ derive( PartialEq, Debug ) ]
struct Struct1
{
  a : i32,
}

pub trait AsNumber
{
  fn make( src : i32 ) -> Self;
  fn as_number( self ) -> i32;
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
