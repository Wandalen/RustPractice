use std::collections::HashMap;

fn main()
{

  let mut hashmap : HashMap< String, i32 > = HashMap::new();
  // hashmap.insert( "abc", 13 );

  hashmap.insert( String::from( "abc" ), 13 );
  hashmap.insert( String::from( "def" ), 14 );

  {
    let key = String::from( "ghi" );
    hashmap.insert( key, 15 );
  }

  // {
  //   let key = "jkl";
  //   hashmap.insert( key, 16 );
  // }
  // ! mismatched types

  for ( k, v ) in &hashmap
  {
    println!( "{} : {}", k, v );
  }
  // < def : 14
  // < ghi : 15
  // < abc : 13

  match hashmap.get( &String::from( "x" ) )
  {
    Some( val ) => println!( "found : x : {}", val ),
    _ => println!( "found : nothing" ),
  }
  // < found : nothing

  match hashmap.get( "x" )
  {
    Some( val ) => println!( "found : x : {}", val ),
    _ => println!( "found : nothing" ),
  }
  // < found : nothing

  hashmap.remove( &String::from( "ghi" ) );
  hashmap.remove( "abc" );
  for ( k, v ) in &hashmap
  {
    println!( "{} : {}", k, v );
  }
  // < def : 14

  match hashmap.get( &String::from( "def" ) )
  {
    Some( val ) => println!( "found : def : {}", val ),
    _ => println!( "found : nothing" ),
  }
  // < found : def : 14

  if let Some( val ) = hashmap.get( &String::from( "def" ) )
  {
    println!( "found : def : {}", val );
  }
  else
  {
    println!( "found : nothing" );
  }
  // < found : def : 14

}
