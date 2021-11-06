// #![ feature( specialization ) ]

//

pub trait FromMaybe< Src > : Sized
{
  fn from_maybe( _: Src ) -> Option< Self >;
}

//

impl< Src : Sized, FromSrc > FromMaybe< Src >
for FromSrc
{
  default fn from_maybe( _src : Src ) -> Option< Self >
  {
    None
  }
}

//

impl< Src : Sized, FromSrc : From< Src > > FromMaybe< Src >
for FromSrc
{
  fn from_maybe( src : Src ) -> Option< Self >
  {
    Some( src.into() )
  }
}

//

pub trait IntoMaybe< Dst > : Sized
{
  fn into_maybe( self ) -> Option< Dst >;
}

//

impl< Src, Dst > IntoMaybe< Dst > for Src
where
  Dst : FromMaybe< Src >,
{
  fn into_maybe( self ) -> Option< Dst >
  {
    Dst::from_maybe( self )
  }
}
