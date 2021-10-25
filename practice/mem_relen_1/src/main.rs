fn main()
{

  let mut string3 = "string3".to_string();
  let mut string4 = "string4".to_string();
  println!( "&string3 : {:p} - stack segment", &string3 );
  println!( "string3.as_ptr : {:p} - heap segment", string3.as_ptr() );
  println!( "string3.capacity : {}", string3.capacity() );
  println!( "string3.len : {}", string3.len() );
  println!( "&string4 : {:p} - stack segment", &string4 );
  println!( "string4.as_ptr : {:p} - heap segment", string4.as_ptr() );
  println!( "string4.capacity : {}", string4.capacity() );
  println!( "string4.len : {}", string4.len() );
  println!( "" );
  // < &string3 : 0x7ffe02f6e978 - stack segment
  // < string3.as_ptr : 0x55d846136ae0 - heap segment
  // < string3.capacity : 7
  // < string3.len : 7
  // < &string4 : 0x7ffe02f6e990 - stack segment
  // < string4.as_ptr : 0x55d846136b00 - heap segment
  // < string4.capacity : 7
  // < string4.len : 7

  string3.push_str( "_abcdefghijklmnopqrstuvxyz" );
  string4.push_str( "_abcdefghijklmnopqrstuvxyz" );
  println!( "&string3 : {:p} - stack segment", &string3 );
  println!( "string3.as_ptr : {:p} - heap segment", string3.as_ptr() );
  println!( "string3.capacity : {}", string3.capacity() );
  println!( "string3.len : {}", string3.len() );
  println!( "&string4 : {:p} - stack segment", &string4 );
  println!( "string4.as_ptr : {:p} - heap segment", string4.as_ptr() );
  println!( "string4.capacity : {}", string4.capacity() );
  println!( "string4.len : {}", string4.len() );
  println!( "" );
  // < &string3 : 0x7ffe02f6e978 - stack segment
  // < string3.as_ptr : 0x55d846136b20 - heap segment
  // < string3.capacity : 33
  // < string3.len : 33
  // < &string4 : 0x7ffe02f6e990 - stack segment
  // < string4.as_ptr : 0x55d846136b50 - heap segment
  // < string4.capacity : 33
  // < string4.len : 33

}