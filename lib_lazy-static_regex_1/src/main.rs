#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

fn extract_day( date : &str ) -> Option< &str >
{

  lazy_static!
  {
    static ref RE : Regex = Regex::new( r"(\d+).(\d+).(\d+)" ).expect( "Failed to compile regexp" );
  };

  RE
  .captures( date )
  .and_then( | cap | cap.get( 2 ).map( | day | day.as_str() ) )
}

//

fn main()
{

  let date = "2001-02-03";
  if let Some( day ) = extract_day( date )
  {
    println!( "Day is {}", day );
  }
  // < Day is 02

}