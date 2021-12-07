
fn main()
{

  let _f1 = | a : i32 |
  {
    println!( "{}", a );
  };

  let _f2 = | a : i32, b : u32 |
  {
    println!( "{} {}", a, b );
  };

  assert_eq!( _f1.is(), true );
  assert_eq!( f1.is(), true );
  assert_eq!( f1f.is(), true );

  assert_eq!( _f2.is(), true );
  assert_eq!( f2.is(), true );
  assert_eq!( f2f.is(), true );

}

//

fn f0() {}
fn f0f() -> bool { false }
fn f1( _ : i32 ) {}
fn f1f( _ : i32 ) -> bool { false }
fn f2( _ : i32, _ : i64 ) {}
fn f2f( _ : i32, _ : i64 ) -> bool { false }

//

trait IntoRoutine< Arg1, Arg2 >
{
  fn is( &self ) -> bool;
  fn _arg1( _a : Arg1 ){}
  fn _arg2( _a : Arg2 ){}
}

//

impl< Arg1, Res, Ro > IntoRoutine< Arg1, () >
for Ro
where
  Ro : Fn( Arg1, () ) -> Res,
{
  fn is( &self ) -> bool
  {
    true
  }
}

//

impl< Arg1, Arg2, Res, Ro > IntoRoutine< Arg1, Arg2 >
for Ro
where
  Ro : Fn( Arg1, Arg2 ) -> Res,
{
  fn is( &self ) -> bool
  {
    true
  }
}
