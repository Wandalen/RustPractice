fn main()
{
  let mut vector = vec![ 1, 2 ];
  let a = &vector[ 0 ];
  // < vector.push( 3 );
  println!( "a : {}", a );
  // ! error[E0502]: cannot borrow `vector` as mutable because it is also borrowed as immutable
}
