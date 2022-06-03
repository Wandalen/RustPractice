
#[ derive( Debug ) ]
struct Single< E >( E );

//

impl< E > From< E > for Single< E >
{
  #[ inline ]
  fn from( src : E ) -> Self
  {
    Self( src )
  }
}

//

impl< T > From< &T > for Single< T >
where
  T : Clone + Into< Self >,
{
  fn from( src : &T ) -> Self
  {
    src.clone().into()
  }
}

// impl< E, T > From< &T > for Single< E >
// where
//   E : Clone,
//   Self : From< E >,
// {
//   #[ inline ]
//   fn from( src : &E ) -> Self
//   {
//     From::from( ( *src ).clone() )
//   }
// }

//

// impl< E > From< &E > for Single< E >
// where
//   E : Clone,
//   Self : From< E >,
// {
//   #[ inline ]
//   fn from( src : &E ) -> Self
//   {
//     From::from( ( *src ).clone() )
//   }
// }

//

fn main()
{

  let a : Single< i32 > = Single::from( 13 );
  dbg!( &a );

  let a : Single< i32 > = Single::from( &13 );
  dbg!( &a );

  let a : Single< i32 > = Single::from( &&13 );
  dbg!( &a );

  let a : Single< i32 > = Single::from( &&&13 );
  dbg!( &a );

}
