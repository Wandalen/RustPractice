fn main()
{
  let v1 = vec![ 1, 2, 3 ];
  println!( "all elements of {:?} are greater than zero : {}", v1, v1.iter().all( | &e | e > 0 ) );
  // : all elements of [1, 2, 3] are greater than zero : true
  let v2 = [ 1, 2, -3 ];
  println!( "all elements of {:?} are greater than zero : {}", v2, v2.iter().all( | &e | e > 0 ) );
  // : all elements of [1, 2, -3] are greater than zero : false
}
