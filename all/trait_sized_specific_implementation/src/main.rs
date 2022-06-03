// No sulution without features.

trait Trait1< T >
{
  fn get( src : &T ) -> usize;
}

impl< T > Trait1< T > for T
where
  T : Sized,
{
  fn get( src : &T ) -> usize
  {
    0
  }
}

impl< T > Trait1< T > for T
where
  T : ?Sized,
{
  fn get( src : &T ) -> usize
  {
    0
  }
}

//

fn main()
{

}
