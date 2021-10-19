
/*
using macro to implement hashmap! macro
*/

macro_rules! hashmap
{
  ( $( $key : expr => $val : expr ),* $(,)? ) =>
  {{
    #[ allow( unused_mut ) ]
    let mut hashmap = std::collections::hash_map::HashMap::new();
    $( hashmap.insert( $key, $val ); )*
    hashmap
  }}
}

//

fn main()
{
  let hashmap = hashmap!{ "key1" => 13, "key2" => 113 };
  println!( "{:?}", hashmap );
}

//

#[test]
fn empty()
{

  let got : std::collections::hash_map::HashMap< &str, u32 > = hashmap!{};
  assert_eq!( got.values().count(), 0 );

}

//

#[test]
fn single()
{

  let got = hashmap!{ "key1" => 13 };
  assert_eq!( got.get( "key1" ), Some( &13 ) );
  assert_eq!( got.values().count(), 1 );

  let got = hashmap!{ "key1" => 13, };
  assert_eq!( got.get( "key1" ), Some( &13 ) );
  assert_eq!( got.values().count(), 1 );

}

//

#[test]
fn several()
{

  let got = hashmap!{ "key1" => 13, "key2" => 113 };
  assert_eq!( got.values().count(), 2 );
  assert_eq!( got.get( "key1" ), Some( &13 ) );
  assert_eq!( got.get( "key2" ), Some( &113 ) );

}
