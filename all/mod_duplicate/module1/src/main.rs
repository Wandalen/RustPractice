use rand::Rng;
use module2::rand1;

fn main()
{
  let mut rng = rand::thread_rng();
  let number : f64 = rng.gen();
  println!( "module1 : rand 0.8.4 : {}", number );
  println!( "module1 : rand 0.7.0 : {}", rand1() );
}