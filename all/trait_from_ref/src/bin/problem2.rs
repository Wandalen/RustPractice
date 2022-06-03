
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
