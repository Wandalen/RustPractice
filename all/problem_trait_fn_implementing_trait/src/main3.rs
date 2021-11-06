#![ allow( dead_code ) ]
#![ allow( unused_variables ) ]
#![ feature( type_name_of_val ) ]

mod wt;

fn main()
{

  assert_eq!( is1( f0 as fn() ), true );
  assert_eq!( is2( f0 as fn() ), true );
  assert_eq!( is2( f0 ), true );

  let r = into_routine_1( f0 );

  // let r = IntoRoutine3::into_routine_3( f0 );
  // is3( f0 );

  // assert_eq!( is4( ( f0 as fn() ).into_routine_4() ), true );
  // assert_eq!( is4( ( f0 ).into_routine_4() ), true );

  let _f = ||
  {
    println!( "hello" );
  };

  let fn_context = vec!( 1, 2, 3 );
  let _fn = ||
  {
    println!( "hello {:?}", fn_context );
  };

  let mut fn_mut_context = vec!( 1, 2, 3 );
  let _fn_mut = ||
  {
    fn_mut_context[ 0 ] = 3;
    println!( "{:?}", fn_mut_context );
  };

  let mut fn_once_context = vec!( 1, 2, 3 );
  let _fn_once = ||
  {
    fn_once_context[ 0 ] = 3;
    let x = fn_once_context;
    println!( "{:?}", x );
  };

  // assert_eq!( is2( (_fn ).into_routine_4() ), true );

}

//

trait Trait1 {}

impl Trait1 for fn () -> () {}

fn f0() -> () {}

fn is1< T : Trait1 >( _ : T ) -> bool
{
  true
}

// fn is2( _ : fn() -> () ) -> bool
// {
//   true
// }

fn is2< R >( _ : fn() -> R ) -> bool
{
  true
}

fn into_routine_1< R >( src : fn() -> R ) -> Routine< fn() -> R >
{
  inspect_logging_type_of!( src );
  Routine { e : src }
}

trait IntoRoutine3< Ro >
{
  fn into_routine_3( src : Ro ) -> Routine< Ro >;
}

type F0< R > = fn() -> R;

impl< R > IntoRoutine3< F0< R > >
for F0< R >
{
  fn into_routine_3( src : F0< R > ) -> Routine< F0< R > >
  {
    Routine { e : src }
  }
}

fn is3< R, IR : IntoRoutine3< F0< R > > >( _ : IR ) -> bool
{
  true
}

// fn into_routine_1b< A1, R >( src : fn( A1 ) -> R ) -> Routine< fn( A1 ) -> R >
// {
//   // inspect_logging_type_of!( src );
//   Routine { e : src }
// }

fn is4< R >( _ : Routine< R > ) -> bool
{
  true
}

// enum Function< R >
// {

//   F0( fn() -> R ),
//   Fn0( Fn() -> R ),

// }

struct Routine< Ro >
{
  pub e : Ro
}

trait IntoRoutine4< Ro >
{
  fn into_routine_4( &self ) -> Routine< Ro >;
}

// impl< Ro, R > IntoRoutine4< fn() -> R >
// for Ro
// {
//   fn into_routine_4( &self ) -> Routine< fn() -> R >
//   {
//     Routine{ e : ( self as fn() -> R ) }
//   }
// }

impl< R > IntoRoutine4< fn() -> R >
for fn() -> R
{
  fn into_routine_4( &self ) -> Routine< fn() -> R >
  {
    Routine{ e : *self }
  }
}

// impl< R, F > IntoRoutine4< R >
// for F
// where
//  F : Fn() -> R
// {
//   fn into_routine_4( &self ) -> Function< R >
//   {
//     Function::Fn0( self )
//   }
// }
