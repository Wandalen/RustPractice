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

  for thread in threads
  {
    thread.join().unwrap();
  }

  /*
  main thread is waiting for termination of children
  */
}
