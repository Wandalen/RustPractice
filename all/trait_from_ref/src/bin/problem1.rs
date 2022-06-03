
#[ derive( Debug ) ]
struct Single( i32 );

//

impl From< i32 > for Single
{
  #[ inline ]
  fn from( src : i32 ) -> Self
  {
    Self( src )
  }
}

//

fn main()
{

  let a = Single::from( 13 );
  dbg!( &a );

  let a = Single::from( &13 );
  dbg!( &a );

  let a = Single::from( &&13 );
  dbg!( &a );

  let a = Single::from( &&&13 );
  dbg!( &a );

}
