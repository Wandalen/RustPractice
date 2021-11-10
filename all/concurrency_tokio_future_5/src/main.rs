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

struct SlowRead< 'a, R >
{
  pub sleep : Sleep,
  pub reader : &'a mut R,
}

//

impl< 'a, R > SlowRead< 'a, R >
{
  pub fn new( reader : &'a mut R ) -> Self
  {
    let sleep = tokio::time::sleep( Duration::from_secs( 1 ) );
    Self { sleep, reader }
  }
}

//

impl< 'a, R > AsyncRead for SlowRead< 'a, R >
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
