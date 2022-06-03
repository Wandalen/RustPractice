
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

impl From< &i32 > for Single
{
  #[ inline ]
  fn from( src : &i32 ) -> Self
  {
    Self( *src )
  }
}

//

impl< T > From< &&T > for Single
where
  T : Clone,
  Self : From< T >
{
  #[ inline ]
  fn from( src : &&T ) -> Self
  {
    From::from( ( **src ).clone() )
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

  let a = Single::from( &&&&13 );
  dbg!( &a );

}
