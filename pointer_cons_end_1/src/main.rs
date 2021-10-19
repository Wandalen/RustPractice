
#[ derive( Debug ) ]
enum List1
{
  Cons( i32, Box< List1 > ),
  End,
}

//

fn main()
{
  // construct a list with help of enum and pointer
  let list1 = List1::Cons( 1, Box::new( List1::Cons( 2, Box::new( List1::End ) ) ) );
  println!( "list1 : {:?}", list1 );
}
