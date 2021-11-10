use std::marker::Unpin;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;
use tokio::time::Sleep;
use tokio::io::AsyncRead;
use tokio::io::ReadBuf;
use tokio::fs::File;
use tokio::time::Instant;
use tokio::io::AsyncReadExt;
use pin_project::pin_project;
use futures::Future;

//

#[ tokio::main( flavor = "current_thread" ) ]
async fn main() -> Result< (), tokio::io::Error >
{
  let mut buff = vec![ 0_u8; 128*1024 ].into_boxed_slice();
  let mut file1 = File::open( "/dev/urandom" ).await?;
  let f1 = SlowRead::new( &mut file1 );
  pin_utils::pin_mut!( f1 );

  let time = Instant::now();
  f1.read_exact( &mut buff ).await?;
  println!( "Read {} in {:?}", buff.len(), time.elapsed() );

  let time = Instant::now();
  f1.read_exact( &mut buff ).await?;
  println!( "Read {} in {:?}", buff.len(), time.elapsed() );

  Ok( () )
}

//

#[pin_project]
struct SlowRead< R >
{
  #[pin]
  pub sleep : Sleep,
  #[pin]
  pub reader : R,
}

//

impl< R > SlowRead< R >
{

  pub fn new( reader : R ) -> Self
  {
    let sleep = tokio::time::sleep( Duration::from_secs( 1 ) );
    Self { sleep, reader }
  }

}

//

impl< R > AsyncRead for SlowRead< R >
where
  R : AsyncRead + Unpin,
{

  fn poll_read
  (
    self : Pin< &mut Self >,
    context : &mut Context< '_ >,
    buff : &mut ReadBuf< '_ >,
  )
  -> Poll< std::io::Result< () > >
  {
    println!( "poll_read" );

    let mut this = self.project();

    if this.sleep.as_mut().poll( context ) == Poll::Pending
    {
      return Poll::Pending
    }

    this.sleep.reset( Instant::now() + Duration::from_millis( 25 ) );
    this.reader.poll_read( context, buff )

  }

}
