use std::thread;
use std::sync::mpsc;
use std::time::Duration;

//

fn main()
{
  let ( tx, rx ) = mpsc::channel();
  /* mpsc stands for multiple producer single consumer */

  for tid in 1..=32
  {
    thread_run( tid, tx.clone() );
  }

  /* children send messages and main thread recieve them all */
  for msg in rx.iter().take( 32 )
  {
    println!( "Thread#0 received {}", msg );
  }

}

//

fn thread_run( tid : usize, tx : mpsc::Sender<usize> ) -> thread::JoinHandle<()>
{
  thread::spawn( move ||
  {
    println!( "Thread#{} started", tid );
    thread::sleep( Duration::from_millis( tid as u64 * 10 ) );
    println!( "Thread#{} sent data", tid );
    tx.send( tid ).unwrap();
  })
}