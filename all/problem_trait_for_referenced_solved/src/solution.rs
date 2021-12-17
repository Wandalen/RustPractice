
fn main()
{
  let u32 = 10;
  println!( "{}", Data( &u32 ).act1() );
  println!( "{}", Data( &&u32 ).act1() );
  println!( "{}", Data( &&&u32 ).act1() );
  println!( "{}", Data( &&&&u32 ).act1() );
}

//

pub struct Data< 'a, T >( &'a T );

//

trait Trait0 {}
impl Trait0 for u32 {}
// impl Trait0 for &u32 {}
impl< T : Trait0 > Trait0 for &T {}

//

trait Trait1
{
  fn act1( self ) -> i32;
}

//

impl< 'a, T > Trait1
for Data< 'a, T >
where T : Trait0,
{
  fn act1( self ) -> i32
  {
    13
  }
}
