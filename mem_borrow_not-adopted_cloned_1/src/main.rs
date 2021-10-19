fn main()
{

  let slice1 = "abc";
  println!( "slice1 : {:p} - data segment", slice1 );
  let string1 = slice1.to_string();
  println!( "string1 : {:p} - heap segment", string1.as_ptr() );
  // < slice1 : 0x55e9e2ff46ed - data segment
  // < string1 : 0x55e9e4680be0 - data segment

}
