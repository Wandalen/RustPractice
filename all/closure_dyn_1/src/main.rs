#![allow(dead_code)]
#![feature(type_name_of_val)]

mod wt;

fn main()
{

  // _fn_experiment1();
  _fn_experiment2();

}

//

// fn _fn_experiment1()
// {

//   fn function1() -> bool { true }

//   let _f = ||
//   {
//     println!( "hello" );
//   };

//   let fn_context = vec!( 1, 2, 3 );
//   let _fn = ||
//   {
//     println!( "hello {:?}", fn_context );
//   };

//   let mut fn_mut_context = vec!( 1, 2, 3 );
//   let _fn_mut = ||
//   {
//     fn_mut_context[ 0 ] = 3;
//     println!( "{:?}", fn_mut_context );
//   };

//   let mut fn_once_context = vec!( 1, 2, 3 );
//   let _fn_once = ||
//   {
//     fn_once_context[ 0 ] = 3;
//     let x = fn_once_context;
//     println!( "{:?}", x );
//   };

//   assert_eq!( is_f( function1 ), true );
//   assert_eq!( is_fn( &function1 ), true );
//   assert_eq!( is_fn_mut( &function1 ), true );
//   assert_eq!( is_fn_once( &function1 ), true );

//   assert_eq!( is_f( _f ), true );
//   assert_eq!( is_fn( &_f ), true );
//   assert_eq!( is_fn_mut( &_f ), true );
//   assert_eq!( is_fn_once( &_f ), true );

//   // assert_eq!( is_f( _fn ), true );
//   assert_eq!( is_fn( &_fn ), true );
//   assert_eq!( is_fn_mut( &_fn ), true );
//   assert_eq!( is_fn_once( &_fn ), true );

//   // assert_eq!( is_f( _fn_mut ), true );
//   // assert_eq!( is_fn( &_fn_mut ), true );
//   assert_eq!( is_fn_mut( &_fn_mut ), true );
//   assert_eq!( is_fn_once( &_fn_mut ), true );

//   // assert_eq!( is_f( _fn_once ), true );
//   // assert_eq!( is_fn( &_fn_once ), true );
//   // assert_eq!( is_fn_mut( &_fn_once ), true );
//   assert_eq!( is_fn_once( &_fn_once ), true );

//   fn is_f < R >                             ( _x : fn() -> R )      -> bool { true }
//   fn is_fn < R, F : Fn() -> R >             ( _x : &F )             -> bool { true }
//   fn is_fn_mut < R, F : FnMut() -> R >      ( _x : &F )             -> bool { true }
//   fn is_fn_once < R, F : FnOnce() -> R >    ( _x : &F )             -> bool { true }

// }

//

fn _fn_experiment2()
{

  fn function1() -> bool { true }

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

  let boxed = Box::new( _f );
  // assert_eq!( is_f( &f1 ), true );
  assert_eq!( is_fn( &boxed ), true );
  assert_eq!( is_fn_mut( &boxed ), true );
  assert_eq!( is_fn_once( &boxed ), true );

  // let rced : std::rc::Rc< dyn Fn() > = std::rc::Rc::new( _f );
  // inspect_logging_type_of!( &rced );
  // assert_eq!( is_f( &f1 ), true );
  // assert_eq!( is_fn( &rced ), true );
  // assert_eq!( is_fn_mut( &rced ), true );
  // assert_eq!( is_fn_once( &rced ), true );

  // let arced : std::sync::Arc< dyn Fn() > = std::sync::Arc::new( _f );
  // inspect_logging_type_of!( &rced );
  // assert_eq!( is_f( &f1 ), true );
  // assert_eq!( is_fn( &arced ), true );
  // assert_eq!( is_fn_mut( &arced ), true );
  // assert_eq!( is_fn_once( &arced ), true );

  fn is_f < R >                             ( _x : fn() -> R )      -> bool { true }
  fn is_fn < R, F : Fn() -> R >             ( _x : &F )             -> bool { true }
  fn is_fn_mut < R, F : FnMut() -> R >      ( _x : &F )             -> bool { true }
  fn is_fn_once < R, F : FnOnce() -> R >    ( _x : &F )             -> bool { true }

}
