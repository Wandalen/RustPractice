#[macro_use]
extern crate lazy_static;

use std::sync::RwLock;

lazy_static!
{
  static ref CLIENTS : RwLock< Vec< String > > =
  {
    RwLock::new( Vec::new() )
  };
}

//

fn main()
{
  CLIENTS
  .write()
  .expect( "Failed to unlcok" )
  .push( "192.168.0.2".to_string() )
  ;

  CLIENTS
  .write()
  .expect( "Failed to unlcok" )
  .push( "192.168.0.3".to_string() )
  ;

  let clients = CLIENTS
  .read()
  .expect( "Failed to unlock" )
  ;

  let first_client = clients.get( 0 ).expect( "No clients" );
  println!( "First client {}", first_client );
  // < First client 192.168.0.2

}