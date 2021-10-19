#![allow(unused_mut)]

use std::mem::size_of;
use std::mem::size_of_val;

fn main()
{

  println!( "size_of( String ) : {}", size_of::<String>() );
  println!( "size_of( RawVec ) : {}", size_of::<std::vec::Vec<i32>>() );
  println!( "size_of( alloc::System ) : {}", size_of::<std::alloc::System>() );
  println!( "size_of( i32 ) : {}", size_of::< i32 >() );
  let a = 13;
  println!( "size_of_val( a ) : {}", size_of_val( &a ) );
  println!( "" );
  // < size_of( String ) : 24
  // < size_of( RawVec ) : 24
  // < size_of( alloc::System ) : 0
  // < size_of( i32 ) : 4
  // < size_of_val( a ) : 4

  let a = 13;
  let string1 = "string1".to_string();
  let string2 = "string2".to_string();
  let mut string3 = "string3".to_string();
  println!( "&a : {:p} - stack segment", &a );
  println!( "&string1 : {:p} - stack segment", &string1 );
  println!( "&string2 : {:p} - stack segment", &string2 );
  println!( "&string3 : {:p} - stack segment", &string3 );
  println!( "string1.as_ptr : {:p} - heap segment", string1.as_ptr() );
  println!( "string2.as_ptr : {:p} - heap segment", string2.as_ptr() );
  println!( "string3.as_ptr : {:p} - heap segment", string3.as_ptr() );
  println!( "" );
  // < &a : 0x7ffd19072214 - stack segment
  // < &string1 : 0x7ffd19072218 - stack segment
  // < &string2 : 0x7ffd19072230 - stack segment
  // < &string3 : 0x7ffd19072248 - stack segment
  // < string1.as_ptr : 0x55ee4b18cae0 - heap segment
  // < string2.as_ptr : 0x55ee4b18cb00 - heap segment
  // < string3.as_ptr : 0x55ee4b18cb20 - heap segment

  let mut string3 = "string3".to_string();
  let mut string4 = "string4".to_string();
  println!( "&string4 : {:p} - stack segment", &string4 );
  println!( "string4.as_ptr : {:p} - heap segment", string4.as_ptr() );
  println!( "string4.capacity : {}", string4.capacity() );
  println!( "string4.len : {}", string4.len() );
  string3.push_str( "_abcdefghijklmn" );
  string4.push_str( "_abcdefghijklmn" );
  println!( "&string4 : {:p} - stack segment", &string4 );
  println!( "string4.as_ptr : {:p} - heap segment", string4.as_ptr() );
  println!( "string4.capacity : {}", string4.capacity() );
  println!( "string4.len : {}", string4.len() );
  println!( "" );
  // < &string4 : 0x7ffd190724c0 - stack segment
  // < string4.as_ptr : 0x55ee4b18cb40 - heap segment
  // < string4.capacity : 7
  // < string4.len : 7
  // < &string4 : 0x7ffd190724c0 - stack segment
  // < string4.as_ptr : 0x55ee4b18cb40 - heap segment
  // < string4.capacity : 22
  // < string4.len : 22

  let string_slice = "jkl";
  println!( "string_slice : {:p} - data segment", string_slice );
  let string5 = string_slice.to_string();
  println!( "string5.as_ptr : {:p} - heap segment", string5.as_ptr() );
  println!( "string_slice : {:p} - data segment", string_slice );
  println!( "" );
  // < string_slice : 0x55ee4ab0840a - data segment
  // < string5.as_ptr : 0x55ee4b18cb60 - heap segment
  // < string_slice : 0x55ee4ab0840a - data segment

  let s: &str = "abc";
  let ss: String = s.to_owned();
  println!( "s : {:p} - data segment", s );
  println!( "ss.as_ptr : {:p} - heap segment", ss.as_ptr() );
  println!( "size_of_val( s ) : {}", size_of_val( s ) );
  println!( "size_of_val( ss ) : {}", size_of_val( &ss ) );
  let v: &[i32] = &[1, 2];
  let vv: Vec<i32> = v.to_owned();
  println!( "v : {:p} - data segment", v );
  println!( "vv.as_ptr : {:p} - heap segment", vv.as_ptr() );
  println!( "size_of_val( v ) : {}", size_of_val( v ) );
  println!( "size_of_val( vv ) : {}", size_of_val( &vv ) );
  println!( "" );
  // < s : 0x55ee4ab0842d - data segment
  // < ss.as_ptr : 0x55ee4b18cb80 - heap segment
  // < size_of_val( s ) : 3
  // < size_of_val( ss ) : 24
  // < v : 0x55ee4ab082b8 - data segment
  // < vv.as_ptr : 0x55ee4b18cba0 - heap segment
  // < size_of_val( v ) : 8
  // < size_of_val( vv ) : 24

  const CONST1 : i32 = 13;
  println!( "CONST1 : {:p} - data segment", &CONST1 );
  println!( "" );
  // < CONST1 : 0x55ee4ab08438 - data segment

  let slice1 = "abc";
  println!( "slice1 : {:p} - data segment", slice1 );
  let string1 = slice1.to_string();
  println!( "string1 : {:p} - heap segment", string1.as_ptr() );
  // < slice1 : 0x55e9e2ff46ed - data segment
  // < string1 : 0x55e9e4680be0 - heap segment

}