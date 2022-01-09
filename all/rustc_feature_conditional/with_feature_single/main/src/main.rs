#![ allow( unused_variables ) ]
#![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]

fn main()
{
  let i : i64 = 13;
  #[ cfg( feature = "nightly" ) ]
  println!( "{} is {}", i, std::any::type_name_of_val( &i ) );
}
