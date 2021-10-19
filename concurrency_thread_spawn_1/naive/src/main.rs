use std::thread;

fn main()
{
  let mut threads = vec![];

  for tid in 0..32
  {
    threads.push( thread::spawn( move ||
    {
      println!( "Thread id : {}", tid );
    }));
  }

  println!( "Main thread" );

  /*
  some thread will not get chance to prin information about itself
  because the main thread does not wait for termination of children
  */
}
