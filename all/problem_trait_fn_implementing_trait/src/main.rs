// #![ allow( dead_code ) ]
// #![ allow( unused_variables ) ]
// #![ feature( type_name_of_val ) ]

mod wt;

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

  // assert_eq!( is1( _f0 ), true );
  // assert_eq!( is1( f0 ), true );

  assert_eq!( _f0.is(), true );
  assert_eq!( f0.is(), true );
  assert_eq!( f0f.is(), true );

  // assert_eq!( _f1.is(), true );
  // assert_eq!( f1.is(), true );
  // assert_eq!( f1f.is(), true );

}

//

// fn is1< Res, F : Fn() -> Res >( _ : F ) -> bool { true }

fn f0() {}
fn f0f() -> bool { false }
fn f1( _ : i32 ) {}
fn f1f( _ : i32 ) -> bool { false }
// fn f2( _ : i32, _ : u32 ) {}
// fn f2f( _ : i32, _ : u32 ) -> bool { false }

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

// impl< Arg1, Res, Ro > IntoRoutine
// for Ro
// where
//   Ro : Fn( Arg1 ) -> Res,
// {
//   fn is( &self ) -> bool
//   {
//     true
//   }
// }
