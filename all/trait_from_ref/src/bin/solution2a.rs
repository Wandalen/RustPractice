
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

impl< E > From< &E > for Single< E >
where
  E : Clone,
  Self : From< E >,
{
  #[ inline ]
  fn from( src : &E ) -> Self
  {
    From::from( ( *src ).clone() )
  }
}

//

fn main()
{

  let a : Single< i32 > = Single::from( 13 );
  dbg!( &a );

  let a : Single< i32 > = Single::from( &13 );
  dbg!( &a );

//   let a : Single< i32 > = Single::from( &&13 );
//   dbg!( &a );
//
//   let a : Single< i32 > = Single::from( &&&13 );
//   dbg!( &a );

}
