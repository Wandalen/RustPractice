
pub fn solution()
{
  let mut renderer = Render::new( "solution1".to_string() );
  renderer.f_on_update = Box::new( | e | println!( "event from {} dt:{}", e.renderer.name, e.dt ) );
  renderer.update();
}

//

struct UpdateEvent<'a>
{
  dt : f64,
  renderer : &'a Render,
}

//

struct Render
{
  name : String,
  f_on_update : Box< dyn Fn( &mut UpdateEvent ) >,
}

//

impl Render
{
  fn new( name : String ) -> Self
  {
    let f_on_update = | _e : &mut UpdateEvent | {};
    Self { name, f_on_update : Box::new( f_on_update ) }
  }
  fn update( &self )
  {
    let mut e = UpdateEvent { dt : 1.0, renderer : self };
    (self.f_on_update)( &mut e );
  }
}
