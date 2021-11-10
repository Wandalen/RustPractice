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
use futures::FutureExt;

//

#[ tokio::main( flavor = "current_thread" ) ]
async fn main() -> Result< (), tokio::io::Error >
{
  let mut buff = vec![ 0_u8; 128*1024 ].into_boxed_slice();
  let mut f1 = SlowRead::new( File::open( "/dev/urandom" ).await? );
  let time = Instant::now();
  dbg!( &time );
  f1.read_exact( &mut buff ).await?;
  println!( "Read {} in {:?}", buff.len(), time.elapsed() );
  Ok( () )
}

//

struct SlowRead< R >
{
  pub sleep : Pin< Box< Sleep > >,
  pub reader : Pin< Box< R > >,
}

//

impl< R > SlowRead< R >
{
  pub fn new( reader : R ) -> Self
  {
    let sleep = Box::pin( tokio::time::sleep( Duration::from_secs( 1 ) ) );
    let reader = Box::pin( reader );
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
    mut self : Pin< &mut Self >,
    context : &mut Context< '_ >,
    buff : &mut ReadBuf< '_ >,
  )
  -> Poll< std::io::Result< () > >
  {
    println!( "poll_read" );

    if self.sleep.poll_unpin( context ) == Poll::Pending
    {
      return Poll::Pending
    }

    self.sleep.as_mut().reset( Instant::now() + Duration::from_millis( 25 ) );
    self.reader.as_mut().poll_read( context, buff )

  }

}
