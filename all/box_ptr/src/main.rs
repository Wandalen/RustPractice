#![feature(type_name_of_val)]

fn main()
{
  box_experimet();
  option_experimet();
}

fn box_experimet()
{
  println!( "\n= box_experimet" );
  let a : Box< i32 > = Box::new( 1 );

  let a_slice : &[ u8 ; 4 ] = unsafe
  {
    std::mem::transmute::< &i32, &[ u8 ; 4 ] >( a.as_ref() )
  };

  println!( "{:?}", a_slice );
  println!( "{}", a );
  println!( "{:?}", a );

  println!( "+ a : {:p}", a );
  println!( "&a : {:p}", &a );
  println!( "+ a.as_ref() : {:p}", a.as_ref() );
  println!( "&a.as_ref() : {:p}", &a.as_ref() );

  f1( a );
}

fn option_experimet()
{
  println!( "\n= option_experimet" );
  let a : Option< Box< i32 > > = Some( Box::new( 1 ) );

  let a1 = a.as_ref().unwrap();
  let a2 = a.as_ref().unwrap().as_ref();

  wtools::typing::inspect_type_of!( a1 );
  wtools::typing::inspect_type_of!( a2 );

  let a_slice : &[ u8 ; 4 ] = unsafe
  {
    std::mem::transmute::< &i32, &[ u8 ; 4 ] >( a.as_ref().unwrap().as_ref() )
  };

  println!( "{:?}", a_slice );
  println!( "{:?}", a );

  // println!( "a.unwrap() : {:p}", a.unwrap() );
  println!( "&a : {:p}", &a );
  println!( "&a.as_ref() : {:p}", &a.as_ref() );
  println!( "a.as_ref().unwrap() : {:p}", a.as_ref().unwrap() );
  println!( "&a.as_ref().unwrap() : {:p}", &a.as_ref().unwrap() );
  println!( "+ a.as_ref().unwrap().as_ref() : {:p}", a.as_ref().unwrap().as_ref() );
  println!( "&a.as_ref().unwrap().as_ref() : {:p}", &a.as_ref().unwrap().as_ref() );

  f2( a );
}

fn f1( a : Box< i32 > )
{

  println!( "+ a : {:p}", a );
  println!( "&a : {:p}", &a );
  println!( "+ a.as_ref() : {:p}", a.as_ref() );
  println!( "&a.as_ref() : {:p}", &a.as_ref() );

}

fn f2( a : Option< Box< i32 > > )
{

  // println!( "a.unwrap() : {:p}", a.unwrap() );
  println!( "&a : {:p}", &a );
  println!( "&a.as_ref() : {:p}", &a.as_ref() );
  println!( "a.as_ref().unwrap() : {:p}", a.as_ref().unwrap() );
  println!( "&a.as_ref().unwrap() : {:p}", &a.as_ref().unwrap() );
  println!( "+ a.as_ref().unwrap().as_ref() : {:p}", a.as_ref().unwrap().as_ref() );
  println!( "&a.as_ref().unwrap().as_ref() : {:p}", &a.as_ref().unwrap().as_ref() );

}
