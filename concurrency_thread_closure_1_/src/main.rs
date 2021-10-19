use std::thread;
use std::sync::Arc;

fn main()
{
  let vec = vec![ 1, 3 ];

  println!( "before : {:?}", vec );

  let thread = thread::spawn( move ||
  {
    println!( "vector : {:?}", vec );
  });

  // println!( "after : {:?}", vec );
  // ! variable moved due to use in closure

  thread.join().unwrap();
  println!( "Done" );
}
