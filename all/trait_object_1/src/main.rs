fn main()
{

  let ex = ExperimentStruct { x : 13 };
  f1( &ex );
  f2( &ex );
  f3( &ex );

}

//

pub trait ExperimentInterface1
{
  fn f1( &self )
  {
    println!( "ExperimentInterface::f1" );
  }
}

pub trait ExperimentInterface2
{
  fn f2( &self )
  {
    println!( "ExperimentInterface::f1" );
  }
}

pub trait ExperimentInterface3 : ExperimentInterface1 + ExperimentInterface2
{
  fn f3( self )
  where Self : Sized,
  /* marker Sized exclude the method from the trait object */
  {
    println!( "ExperimentInterface3::f3" );
  }
}

pub struct ExperimentStruct
{
  x : i32,
}

impl ExperimentInterface1 for ExperimentStruct
{
  fn f1( &self )
  {
    println!( "ExperimentStruct::f1( {} )", self.x );
  }
}

impl ExperimentInterface2 for ExperimentStruct
{
  fn f2( &self )
  {
    println!( "ExperimentStruct::f2( {} )", self.x );
  }
}

impl< T > ExperimentInterface3 for T
where
  T : ExperimentInterface1 + ExperimentInterface2
{
}

fn f1( src : &dyn ExperimentInterface1 )
{
  src.f1();
}

fn f2( src : &dyn ExperimentInterface2 )
{
  src.f2();
}

fn f3( src : &dyn ExperimentInterface3 )
{
  src.f1();
  src.f2();
}
