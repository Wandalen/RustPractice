fn main()
{

  let default_i8 : i8 = Default::default();
  println!( "default_i8 : {:?}", default_i8 );
  // < default_i8 : 0

  let default_string : String = Default::default();
  println!( "default_string : {:?}", default_string );
  // < default_string : ""

  let default_struct : Context = Default::default();
  println!( "default_struct : {:?}", default_struct );
  // < default_struct : Context { i: 13, f: 4.5, state: Off }

}

//

#[ allow( dead_code ) ]
#[ derive( Debug ) ]
enum State
{
  On,
  Off,
}

//

#[ derive( Debug ) ]
struct Context
{
  i : i32,
  f : f32,
  state : State,
}

//

impl Default for Context
{
  fn default() -> Self
  {
    Self { i : 13, f : 4.5, state : State::Off }
  }
}
