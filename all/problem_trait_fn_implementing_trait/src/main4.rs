
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

  // assert_eq!( _f0.is(), true );
  // assert_eq!( f0.is(), true );
  // assert_eq!( f0f.is(), true );

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
  type Res1;
  type Arg1;
  fn is( &self ) -> bool;
  fn _res( _r : Self::Res1 )
  {
  }
  fn _arg1( _a : Self::Arg1 )
  {
  }
}

//

// impl< Res, Ro > IntoRoutine
// for Ro
// where Ro : Fn() -> Res
// {
//   fn is( &self ) -> bool
//   {
//     true
//   }
// }

trait Ro1< Arg1, Res > : Fn( Arg1 ) -> Res
{
}

//

impl< Arg, Res, Ro > IntoRoutine
// impl< Arg1, Res, Ro : Ro1< Arg1, Res > > IntoRoutine
// for Fn( Arg1 ) -> Res
// for Ro
for Ro
where
  Ro : Fn( Self::Arg1 ) -> Self::Res1,
  // Ro : Fn( Arg1 ) -> Res,
  // Ro : for< Arg1, Res0 > Fn( Arg1 ) -> Res0,
{
  type Res1 = Res;
  type Arg1 = Arg;
  fn is( &self ) -> bool
  {
    true
  }
  fn _res( _r : Self::Res1 )
  {
  }
  fn _arg1( _a : Self::Arg1 )
  {
  }
  // fn _abc( _r : Self::Res )
  // {
  // }
}
