
fn imperative() -> i32
{
  let mut c = 0;
  for i in 1 ..
  {
    let val = i*i;
    if val > 1000
    {
      break;
    }
    else if val % 3 == 0
    {
      c += val;
    }
  }
  c
}

//

fn functional() -> i32
{
  ( 0 .. )
  .map( | i | i*i )
  .take_while( | &val | val <= 1000 )
  .filter( | &val | val % 3 == 0 )
  .fold( 0, | cur, val | cur + val )
}

//

fn main()
{
  println!( "imperative : {}", imperative() );
  // : 3465
  println!( "functional : {}", functional() );
  // : 3465
}
