use std::collections::HashMap;
use std::mem::size_of_val;

fn main()
{
  let mut hashmap : HashMap< &'static str, &'static str > = HashMap::new();
  hashmap.insert( "EUR", "Euro" );
  hashmap.insert( "USD", "US Dollar" );
  hashmap.insert( "CHF", "Swiss Francs" );

  let usd = hashmap.get( "EUR" );
  if let Some( usd ) = usd
  {
    println!( "usd : {}", usd );

    let i = 13;
    println!( "Size of &i32 is {}", size_of_val( &&i ) );
    println!( "Size of &str is {}", size_of_val( usd ) );

  }
  // < usd : Euro
  // < Size of &i32 is 8
  // < Size of &str is 16

}
