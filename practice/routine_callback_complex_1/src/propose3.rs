#![allow(unused)]

pub fn solution()
{

  // let mut renderer = Render::new( "solition2".to_string() );
  // renderer.f_on_update = Box::new( | e | println!( "event from {}", e.renderer.name ) );
  // renderer.update();

  let renderer = Render::new("name1".to_string(), |_| {});
  let mut ue = UpdateEvent { dt: 5. };
  ( renderer.f_on_update )( &mut ue );

}

pub struct UpdateEvent
{
  pub dt: f64,
}

pub struct Render<OnUpdate>
where
  OnUpdate: Fn(&mut UpdateEvent),
{
  pub name: String,
  pub f_on_update: OnUpdate,
}

impl<OnUpdate> Render<OnUpdate>
where
  OnUpdate: Fn(&mut UpdateEvent),
{
    pub fn new(name: String, f: OnUpdate) -> Render<OnUpdate>
    {
        Render
        {
            name,
            f_on_update: f,
        }
    }
}