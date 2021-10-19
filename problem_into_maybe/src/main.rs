#![ feature( specialization ) ]

mod from_maybe;
use from_maybe::*;

fn main()
{
  let u32 : u32 = 13;
  let u64 : u64 = u32.into();
  println!( "u64 : {}", u64 );
  let u8 : u8 = u32 as u8;
  println!( "u8 : {}", u8 );
  let u8 : Option< u8 > = u32.into_maybe();
  println!( "u8 : {:?}", u8 );
}
