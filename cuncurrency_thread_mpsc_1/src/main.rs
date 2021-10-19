use std::thread;
use std::sync::mpsc;

fn main()
{
  let ( tx, rx ) = mpsc::channel();
  /* mpsc stands for multiple producer single consumer */

  thread::spawn( move ||
  {
    tx.send( "abc" ).unwrap();
  });

  /* child thread send a message and main thread recieve it */
  println!( "received {}", rx.recv().unwrap() );
}
