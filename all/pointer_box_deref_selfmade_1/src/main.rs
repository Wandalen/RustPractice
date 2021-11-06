use std::ops::Deref;

fn main()
{

  let a = Box::new( "a" );
  println!( "a : {}", *a );
  print( &a );

  let b = MyBox::new( "b" );
  println!( "b : {}", *b );
  print( &b );

}

//

struct MyBox< T >( T );
impl< T > MyBox< T >
{
  fn new( src : T ) -> MyBox< T >
  {
    MyBox( src )
  }
}

//

impl<T> Deref for MyBox<T>
{
  type Target = T;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

//

fn print( src : &str )
{
  println!( "{}", src );
}