#![ feature( specialization ) ]

fn main()
{
  let u32 : u32 = 13;
  let u64 : u64 = u32.into();
  println!( "u64 : {}", u64 );
  let u8 : u8 = u32 as u8;
  println!( "u8 : {}", u8 );
  let u8 : u8 = u32.into_coercing();
  println!( "u8 : {}", u8 );
}

//

pub trait FromCoercing< Src > : Sized
{
  fn from_coercing( _: Src ) -> Self;
}

//

// impl< Src : Sized, FromSrc : From< Src > > FromCoercing< Src >
// for FromSrc
// {
//   fn from_coercing( src : Src ) -> Self
//   {
//     Self::from( src )
//   }
// }

//

impl FromCoercing< u32 > for u16
{
  default fn from_coercing( src : u32 ) -> Self
  {
    src as u16
  }
}

//

pub trait IntoCoercing< Dst > : Sized
{
  fn into_coercing( self ) -> Dst;
}

//

impl< Src, Dst > IntoCoercing< Dst > for Src
where
  Dst : FromCoercing< Src >,
{
  fn into_coercing( self ) -> Dst
  {
    Dst::from_coercing( self )
  }
}
