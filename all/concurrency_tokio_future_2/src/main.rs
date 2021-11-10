use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;
use tokio::time::Sleep;
use futures::FutureExt;

//

#[ tokio::main( flavor = "current_thread" ) ]
async fn main()
{
  let future1 = MyFuture::new();
  println!( "Awaiting..." );
  future1.await;
  println!( "Done!" );
}

//

struct MyFuture
{
  pub sleep : Pin< Box< Sleep > >
}

impl MyFuture
{
  pub fn new() -> Self
  {
    Self { sleep : Box::pin( tokio::time::sleep( Duration::from_secs( 1 ) ) ), }
  }
}

impl Future for MyFuture
{
  type Output = ();

  fn poll
  (
    mut self : Pin< &mut Self >,
    context : &mut Context< '_ >,
  )
  -> Poll< Self::Output >
  {
    println!( "MyFuture::poll()" );
    // self.sleep.as_mut().poll( context )
    self.sleep.poll_unpin( context )
  }

}
