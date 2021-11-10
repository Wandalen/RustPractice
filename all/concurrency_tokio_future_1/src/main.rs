use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

// fn main()
// {

//   let rt = tokio::runtime::Builder::new_current_thread()
//   .build()
//   .unwrap();
//   let future1 = MyFuture {};
//   rt.block_on( future1 );

// }

#[ tokio::main( flavor = "current_thread" ) ]
async fn main()
{

  let future1 = MyFuture {};
  println!( "Awaiting..." );
  future1.await;
  println!( "Done!" );
}

struct MyFuture {}

impl Future for MyFuture
{
  type Output = ();

  fn poll
  (
    self : Pin< &mut Self >,
    _context : &mut Context< '_ >,
  )
  -> Poll< Self::Output >
  {
    println!( "MyFuture::poll()" );
    Poll::Ready( () )
  }

}
