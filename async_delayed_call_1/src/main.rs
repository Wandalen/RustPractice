
pub struct Object1
{
  k1 : i32,
}

//

impl Object1
{
  pub fn new() -> Object1
  {
    Object1
    {
      k1 : 13
    }
  }
}

//

fn main()
{

  let object1 : Box<Object1> = Box::new( Object1::new() );

  callback_call_1( Box::new( ||
  {
    println!( "{:?}", object1.k1 );
  }));

}

//

fn callback_call_1<F>( mut callback1 : F )
where
  F : FnMut()
{
  println!( "inside callback_call_1" );
  callback1();
  /* how to call callback1 with delay 1 second, not blocking the thread?
  is it possible to call the callback1 in the same thread?
  is it possible to reach similar behavior setTimeout from JavaScript has?
  */
}
