use std::sync::{ mpsc, Arc, Mutex };
use std::thread;
use std::error::Error;
/* xxx : rewrite using more sane scenario */

mod problem;

fn main() 
{
  let ( tx, rx ) = mpsc::sync_channel( 1024 );
  let sh_rx = Arc::new( Mutex::new( rx ) );

  for tid in 1..=4
  {
    thread_performer( tid, sh_rx.clone() );
  }

  thread_producer( tx ).join().unwrap();
}

//

fn prime_is( u : usize ) -> bool
{
  ( 2 .. u ).all( | i | { u % i != 0 } )
}

//

fn thread_producer( tx : mpsc::SyncSender<usize> ) -> thread::JoinHandle<()>
{
  thread::spawn( move ||
  {
    for i in 100_000 .. 150_000
    {
      tx.send( i ).unwrap();
    }
  })
}

//

fn thread_performer( tid : usize, sh_rx : Arc< Mutex< mpsc::Receiver<usize> > > ) 
{
  thread::spawn( move ||
  {
    loop
    {
      let mut n = 0;

      // n = sh_rx.lock()?.try_recv()?;

      // match sh_rx.lock()
      // {
      //   Ok( rx ) => 
      //   {
      //     match rx.try_recv()
      //     {
      //       Ok( _n ) => { n = _n; },
      //       Err( _ ) => (),
      //     }
      //   }
      //   Err( _ ) => (),
      // }

      match sh_rx_get( &sh_rx )
      {
        Ok( _n ) => { n = _n },
        Err( _ ) => (),
      }

      if n != 0 && prime_is( n )
      {
        println!( "Thread::{} found prime {}", tid, n );
      }
    }
  });
}

// enum Error1<'a>
// {
//   TryRecvError( std::sync::mpsc::TryRecvError ),
//   PoisonError( std::sync::PoisonError< std::sync::MutexGuard< a', mpsc::Receiver< usize > > > ),
// }

fn sh_rx_get( sh_rx : &Arc< Mutex< mpsc::Receiver< usize > > > ) -> Result< usize, Box<dyn Error> >
{
  let rx = sh_rx.lock().map_err(|e| e.to_string())?;
  // let rx = sh_rx.lock()?;
  let r = rx.try_recv()?;
  Ok( r )
}
