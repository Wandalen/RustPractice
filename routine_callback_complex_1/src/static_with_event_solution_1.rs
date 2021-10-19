
pub fn solution()
{
  let mut mechanics = Box::new( Mechanics { val : 13. } );
  let mut renderer = Render::new( "solution1".to_string() );
  renderer.f_on_update = Box::new( move | e |
  {
    println!( "mechanics.val : {}", mechanics.val );
    mechanics.val = 3.;
    println!( "dt : {}", e.dt );
  });
  renderer.update();
}

//

struct UpdateEvent<'a>
{
  dt : f64,
  renderer : &'a mut Render,
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
  f_on_update : Box< dyn FnMut( &mut UpdateEvent ) >,
}

//

impl Render
{
  fn new( name : String ) -> Self
  {
    let f_on_update = | _e : &mut UpdateEvent | {};
    Self { name, f_on_update : Box::new( f_on_update ) }
  }
  fn update( &mut self )
  {
    let mut e = UpdateEvent { dt : 1.0, renderer : self };
    (self.f_on_update)( &mut e );
  }
}
