#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static!
{
  static ref CURRENCIES : HashMap< &'static str, &'static str > =
  {
    let mut hashmap = HashMap::new();
    hashmap.insert( "EUR", "Euro" );
    hashmap.insert( "USD", "US Dollar" );
    hashmap.insert( "CHF", "Swiss Francs" );
    hashmap
  };
}

//

fn main()
{
  let usd = CURRENCIES.get( "USD" );
  if let Some( usd ) = usd
  {
    println!( "usd : {}", usd );
  }

  if let Some( chf ) = CURRENCIES.get( "CHF" )
  {
    println!( "CHF stands for {}", chf );
  }

}
