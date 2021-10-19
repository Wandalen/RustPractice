use std::borrow::Cow;
use std::any::type_name;

fn main()
{

  let v1 = modify( &[ 0, 1, 2, 3 ] );
  dbg!( &v1 );
  println!( "{}", type_name_of( &v1 ) );
  match v1
  {
    Cow::Owned( val ) => println!( "Cow::Owned( {:?} ) : {}", &val, type_name_of( &val ) ),
    Cow::Borrowed( val ) => println!( "Cow::Borrowed( {:?} ) : {}", &val, type_name_of( &val ) ),
  }
  println!( "" );

  let v2 = modify( &[ 0, 1, 2, 3, 4, 5, 6 ] );
  dbg!( &v2 );
  println!( "{}", type_name_of( &v2 ) );
  match v2
  {
    Cow::Owned( val ) => println!( "Cow::Owned( {:?} ) : {}", &val, type_name_of( &val ) ),
    Cow::Borrowed( val ) => println!( "Cow::Borrowed( {:?} ) : {}", &val, type_name_of( &val ) ),
  }
  println!( "" );

}

//

fn modify< 'a >( src : &'a [i32] ) -> Cow< 'a, [ i32 ] >
{
  match src.len()
  {
    0..=5 => Cow::Owned( src.to_vec() ),
    _ => Cow::Borrowed( src ),
  }
}

//

fn type_name_of< T >( _ : &T ) -> &'static str
{
  type_name::< T >()
}
