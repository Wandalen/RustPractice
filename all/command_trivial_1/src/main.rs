
use std::process::Command;
use std::io::{ self, Write };

fn main()
{
  // let result = Command::new( "/bin/cat" )
  let result = Command::new( "ls" )
  .arg( "-al" )
  // .arg( "Cargo.toml" )
  .output()
  // .spawn()
  .expect( "failed to execute process" )
  ;

  println!( "result : {:?}", result );
  // println!( "result.status : {:?}", result.status );

  println!( "status: {}", result.status );
  io::stdout().write_all( &result.stdout ).unwrap();
  io::stderr().write_all( &result.stderr ).unwrap();
//
//   assert!( output.status.success() );
}
