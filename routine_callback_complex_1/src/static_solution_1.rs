
pub fn solution()
{
  let mut mechanics = Box::new( Mechanics { val : 13. } );
  let mut renderer = Render::new( "solution1".to_string() );
  renderer.f_on_update = Box::new( move ||
  {
    println!( "mechanics.val : {}", mechanics.val );
  });
  renderer.update();
}

//

struct Mechanics
{
  val : f32,
}

//

struct Render
{
  name : String,
  f_on_update : Box< dyn Fn() >,
}

//

impl Render
{
  fn new( name : String ) -> Self
  {
    let f_on_update = || {};
    Self { name, f_on_update : Box::new( f_on_update ) }
  }
  fn update( &self )
  {
    (self.f_on_update)();
  }
}
