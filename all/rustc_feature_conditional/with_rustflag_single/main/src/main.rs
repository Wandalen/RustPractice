#![ allow( unused_variables ) ]
#![ cfg_attr( nightly, feature( type_name_of_val ) ) ]

///
/// Run it with : `RUSTFLAGS="--cfg nightly" cargo run`
///

fn main()
{
  let i : i64 = 13;
  #[ cfg( nightly ) ]
  println!( "{} is {}", i, std::any::type_name_of_val( &i ) );
}
