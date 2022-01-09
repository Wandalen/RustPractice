#![ allow( unused_variables ) ]
#![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]

#[ cfg( feature = "nightly" ) ]
use module1::name_of;

fn main()
{
  let i : i64 = 13;
  #[ cfg( feature = "nightly" ) ]
  println!( "{} is {}", i, name_of!( i ) );
}
