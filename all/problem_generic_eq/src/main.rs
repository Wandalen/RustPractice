#![feature(specialization)]

fn main()
{

  let a_u32 = 1_u32;
  let a2_u32 = 1_u32;
  let b_u32 = 3_u32;
  let a2_i32 = 1_i32;
  let b_i32 = 3_i32;
  println!( "a_u32 == a2_u32 : {}", a_u32.identical_generic( &a_u32 ) );
  println!( "a_u32 == a2_u32 : {}", a_u32.identical_generic( &a2_u32 ) );
  println!( "a_u32 == b_u32 : {}", a_u32.identical_generic( &b_u32 ) );
  println!( "a_u32 == a2_i32 : {}", a_u32.identical_generic( &a2_i32 ) );
  println!( "a_u32 == b_i32 : {}", a_u32.identical_generic( &b_i32 ) );

  let a_slice = "abc";
  let a2_slice = "abc";
  let b_slice = "def";
  let a2_string = "abc".to_string();
  let b_string = "def".to_string();
  println!( "a_slice == a2_slice : {}", a_slice.identical_generic( &a_slice ) );
  println!( "a_slice == a2_slice : {}", a_slice.identical_generic( &a2_slice ) );
  println!( "a_slice == b_slice : {}", a_slice.identical_generic( &b_slice ) );
  println!( "a_slice == a2_string : {}", a_slice.identical_generic( &a2_string ) );
  println!( "a_slice == b_string : {}", a_slice.identical_generic( &b_string ) );

}

//

pub trait IdenticalGeneric< B >
{
  fn identical_generic( self : &Self, _b : &B ) -> bool
  {
    return false;
  }
}

//

impl< A : PartialEq< B >, B > IdenticalGeneric< B > for A
{
  fn identical_generic( self : &Self, b : &B ) -> bool
  {
    &self == &b
  }
}

//

impl< A, B > IdenticalGeneric< B > for A
{
  default fn identical_generic( self : &Self, _b : &B ) -> bool
  {
    false
  }
}

//
