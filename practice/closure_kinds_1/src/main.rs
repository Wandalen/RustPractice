#![allow(unused_variables)]

//

fn main()
{

  let b = "def".to_string();
  let closure1 = | a |
  {
     println!( "closure : {:?}", a );
     // but returning a compiles for every type of closure
     // a
     b
  };

  fn_once_1( closure1 );
  // < fn_once_1( closure1 );
  // ! use of moved value: `closure1`
  // understandable
  // fn_1( closure1 );
  // ! expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
  // ? but why? and why returning a instead of b from closure1 compiles?
  // fn_mut_1( closure1 );
  // ! expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
  // ? but why? and why returning a instead of b from closure1 compiles?

  let b = "def".to_string();
  let closure_mut_1 = | mut a : String |
  {
     a.push_str( &b );
     println!( "closureMut : {:?}", a );
     a
  };

  fn_once_1( closure_mut_1 );
  fn_once_1( closure_mut_1 );
  fn_1( closure_mut_1 );
  fn_mut_1( closure_mut_1 );

  let mut b = "def".to_string();
  let closure_move_1 = move | mut a : String |
  {
     b.push( 'g' );
     a.push_str( &b );
     println!( "closureMove : {:?}", a );
     a
  };

  // try to uncomment
  // fn_once_1( closure_move_1 );
  // works
  // ? but why? routine write to its closure!

  // try to uncomment
  // fn_1( closure_move_1 );
  // ! this closure implements `FnMut`, not `Fn`
  // ? but why fn_once_1 works?

  // try to uncomment
  // fn_mut_1( closure_move_1 );
  // works as predicted

}

//

// fn type_name_of< T >( _ : &T ) -> &'static str
// {
//   type_name::< T >()
// }

//

fn fn_once_1<F>( callback1 : F )
where
  F : FnOnce( String ) -> String
{
  println!( "in fnOnce" );
  callback1( "abc".to_string() );
}

//

fn fn_1<F>( callback1 : F )
where
  F : Fn( String ) -> String
{
  println!( "in fn" );
  callback1( "abc".to_string() );
}

//

fn fn_mut_1<F>( mut callback1 : F )
where
  F : FnMut( String ) -> String
{
  println!( "in fnMut" );
  callback1( "abc".to_string() );
}

//