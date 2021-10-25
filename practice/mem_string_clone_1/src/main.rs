fn main()
{

  move1();
  clone1();

}

//

fn move1()
{
  println!( "move" );
  let mut string1 = String::from( "string1" );
  println!( "string1 : {} : {:p}", string1, &string1 );
  println!( "&string1[..] : {:p}", &string1[..] );
  let mut string2 = string1;
  // < println!( "string1 : {}", string1, &string1 );
  // ! borrow of moved value: `string1`
  println!( "string2 : {} : {:p}", string2, &string2 );
  println!( "&string2[..] : {:p}", &string2[..] );
  // &string1[..] == &string2[..]
  println!( "" );
}

//

fn clone1()
{
  println!( "clone" );
  let mut string1 = String::from( "string1" );
  let mut string2 = string1.clone();
  println!( "string1 : {} : {:p}", string1, &string1 );
  println!( "&string1[..] : {:p}", &string1[..] );
  println!( "string2 : {} : {:p}", string2, &string2 );
  println!( "&string2[..] : {:p}", &string2[..] );
  // &string1[..] <> &string2[..]
  println!( "" );
}
