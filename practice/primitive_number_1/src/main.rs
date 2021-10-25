
use std::any::type_name;
use std::mem;

//

fn type_name_of< T >( _ : &T ) -> &'static str
{
  type_name::< T >()
}

//

fn main()
{

  let trivial = 13;
  println!( "trivial : {} = {}", type_name_of( trivial ), trivial );
  // : trivial : i32 = 13

  let explicit : i16 = 13;
  println!( "explicit : {} = {}", type_name_of( explicit ), explicit );
  // : explicit : i16 = 13

  let with_underscores = 92__000_000;
  println!( "with_underscores : {} = {}", type_name_of( with_underscores ), with_underscores );
  // : with_underscores : i32 = 92000000

  let with_type = 64_u16;
  println!( "with_type : {} = {}", type_name_of( with_type ), with_type );
  // : with_type : u16 = 64

  let default_for_the_platform : isize = 13;
  println!( "default_for_the_platform : {} = {} -- {} bytes", type_name_of( default_for_the_platform ), default_for_the_platform, mem::size_of_val( &default_for_the_platform ) );
  // : default_for_the_platform : isize = 13 -- 8 bytes

  let hex = 0xff;
  println!( "hex : {} = {}", type_name_of( hex ), hex );
  // : hex : i32 = 255

  let octal = 0o77;
  println!( "octal : {} = {}", type_name_of( octal ), octal );
  // : octal : i32 = 63

  let bin = 0b1101;
  println!( "bin : {} = {}", type_name_of( bin ), bin );
  // : bin : i32 = 13

  let char_like = b'A';
  println!( "char_like : {} = {}", type_name_of( char_like ), char_like );
  // : char_like : u8 = 65

  let float = 13.0;
  println!( "float : {} = {}", type_name_of( float ), float );
  // : float : f64 = 13

  let float_with_only_dot = 13.;
  println!( "float_with_only_dot : {} = {}", type_name_of( float_with_only_dot ), float_with_only_dot );
  // : float_with_only_dot : f64 = 13

  let not_a_number = f32::NAN;
  println!( "not_a_number : {} = {}", type_name_of( not_a_number ), not_a_number );
  // : not_a_number : f32 = NaN

  let infinity = f32::INFINITY;
  println!( "infinity : {} = {}", type_name_of( infinity ), infinity );
  // : infinity : f32 = inf

  // < let incompatible_literal : f32 = 13;
  // < println!( "incompatible_literal : {} = {}", type_name_of( incompatible_literal ), incompatible_literal );
  // ! expected `f32`, found integer

}
