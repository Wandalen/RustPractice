
fn main()
{

  let default_struct : Context = Default::default();
  println!( "default_struct : {:?}", default_struct );
  // < default_struct : Context { i: 13, f: 4.5, state: Off }

  let default_struct_built = Context::default().i( -13 ).f( -4.5 ).form();
  println!( "default_struct_built : {:?}", default_struct_built );

}

//

#[ allow( dead_code ) ]
#[ derive( Debug, Copy, Clone ) ]
enum State
{
  On,
  Off,
}

//

#[ derive( Debug, Copy, Clone ) ]
struct Context
{
  i : i32,
  f : f32,
  state : State,
  formed : i32,
}

//

impl Default for Context
{
  fn default() -> Self
  {
    let context = Self { i : 13, f : 4.5, state : State::Off, formed : 0 };
    context
  }
}

//

impl Context
{
  fn i( &mut self, i : i32 ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.i = i;
    self
  }
  fn f( &mut self, f : f32 ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.f = f;
    self
  }
  #[ allow( dead_code ) ]
  fn state( &mut self, state : State ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.state = state;
    self
  }
  fn form( self ) -> Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    Self { formed : 1, .. self }
  }
}
