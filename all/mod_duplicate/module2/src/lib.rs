use rand::Rng;

pub fn rand1() -> f64
{
  let mut rng = rand::thread_rng();
  let number : f64 = rng.gen();
  number
}
