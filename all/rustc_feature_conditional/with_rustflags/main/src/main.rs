#![ allow( unused_variables ) ]
// #![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]
#![ cfg_attr( nightly, feature( type_name_of_val ) ) ]

// #[ cfg( feature = "nightly" ) ]
#[ cfg( nightly ) ]
use module1::name_of;

fn main()
{
  let i : i64 = 13;
  // #[ cfg( feature = "nightly" ) ]
  #[ cfg( nightly ) ]
  println!( "{} is {}", i, name_of!( i ) );
}
