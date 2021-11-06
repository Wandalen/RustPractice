
fn main()
{

  let _f0 = ||
  {
    println!( "hello" );
  };

  let _f1 = | a : i32 |
  {
    println!( "{}", a );
  };

  let _f2 = | a : i32, b : u32 |
  {
    println!( "{} {}", a, b );
  };

  assert_eq!( _f0.is(), true );
  assert_eq!( f0.is(), true );
  assert_eq!( f0f.is(), true );

  assert_eq!( _f1.is(), true );
  assert_eq!( f1.is(), true );
  assert_eq!( f1f.is(), true );

}

//

fn f0() {}
fn f0f() -> bool { false }
fn f1( _ : i32 ) {}
fn f1f( _ : i32 ) -> bool { false }

//

trait IntoRoutine
{
  fn is( &self ) -> bool;
}

//

impl< Res, Ro > IntoRoutine
for Ro
where Ro : Fn() -> Res
{
  fn is( &self ) -> bool
  {
    true
  }
}

//

impl< Arg1, Res, Ro > IntoRoutine
for Ro
where
  Ro : Fn( Arg1 ) -> Res,
{
  fn is( &self ) -> bool
  {
    true
  }
}
