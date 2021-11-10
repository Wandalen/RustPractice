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
  let f1 = SlowRead::new( File::open( "/dev/urandom" ).await? );
  let time = Instant::now();
  dbg!( &time );
  // let mut f1 = unsafe { Pin::new_unchecked( &mut f1 ) };
  // pin_utils::pin_mut!( f1 );
  let mut f1 = Box::pin( f1 );
  f1.read_exact( &mut buff ).await?;
  println!( "Read {} in {:?}", buff.len(), time.elapsed() );
  Ok( () )
}

//

struct SlowRead< R >
{
  // pub sleep : Pin< Box< Sleep > >,
  pub sleep : Sleep,
  // pub reader : Pin< Box< R > >,
  pub reader : R,
}

//

impl< R > SlowRead< R >
{
  pub fn new( reader : R ) -> Self
  {
    // let sleep = Box::pin( tokio::time::sleep( Duration::from_secs( 1 ) ) );
    let sleep = tokio::time::sleep( Duration::from_secs( 1 ) );
    // let reader = Box::pin( reader );
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

    let ( mut sleep, reader ) = unsafe
    {
      let this = self.get_unchecked_mut();
      (
        Pin::new_unchecked( &mut this.sleep ),
        Pin::new_unchecked( &mut this.reader ),
      )
    };

    if sleep.poll_unpin( context ) == Poll::Pending
    {
      return Poll::Pending
    }

    sleep.reset( Instant::now() + Duration::from_millis( 25 ) );
    reader.poll_read( context, buff )

  }

}
