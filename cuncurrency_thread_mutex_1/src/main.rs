use std::thread;
use std::sync::{ Mutex, Arc };

fn main()
{
  let data = Arc::new( Mutex::new( vec![ 0 ] ) );
  println!( "data @ {:?}", Arc::as_ptr( &data ) );
  let mut threads = vec![];

  for tid in 0 .. 16
  {
    let data = Arc::clone( &data );
    let thread = thread::spawn( move ||
    {
      println!( "thread#{} data @ {:?}", tid, Arc::as_ptr( &data ) );
      let mut d = data.lock().unwrap();
      let e = d[ d.len()-1 ] + 1;
      d.push( e );
    });
    threads.push( thread );
  }

  for thread in threads
  {
    thread.join().unwrap();
  }

  println!( "Done : {:?}", data.lock().unwrap() );
}
