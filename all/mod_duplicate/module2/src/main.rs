use rand::Rng;

pub fn rand1() -> f64
{
  let mut rng = rand::thread_rng();
  let number : f64 = rng.gen();
  number
}

fn main()
{
  let mut rng = rand::thread_rng();
  let number : f64 = rng.gen();
  println!( "rand 0.7.0 : {}", number );
}