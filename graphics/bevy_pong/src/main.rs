use bevy::prelude::*;

///
/// Movable
///

#[ derive( Component ) ]
pub struct Movable
{
  pub speed : f32,
  pub direction : Vec3,
}

///
/// Paddle
///

#[ derive( Component ) ]
pub struct Paddle
{
  pub position : f32,
}

///
/// Setup
///

fn setup( mut commands : Commands )
{

  // 2D orthographic camera
  let camera = OrthographicCameraBundle::new_2d();

  commands.spawn_bundle( camera );

  let size = 25.0;
  let sprite = Sprite { custom_size : Some( Vec2::new( size, size ) ), .. Default::default() };
  commands.spawn_bundle( SpriteBundle
  {
    sprite,
    .. Default::default()
  })
  .insert( Movable { speed : 1.0, direction : Vec3::new( 1.0, 1.0, 0.0 ).normalize() } )
  ;

  let transform = Transform
  {
    translation : Vec3::new( 300.0, 0.0, 0.0 ),
    .. Default::default()
  };
  let sprite = Sprite
  {
    custom_size : Some( Vec2::new( 10.0, 100.0 ) ),
    .. Default::default()
  };
  commands.spawn_bundle( SpriteBundle
  {
    sprite,
    transform,
    .. Default::default()
  })
  .insert( Paddle { position : 0.0 } )
  ;

}

//

pub fn ball_move( mut query : Query<( &Movable, &mut Transform )> )
{

  let (movable, mut transform) = query.single_mut();
  transform.translation += movable.direction * movable.speed;

}

///
/// Print all resources
///

pub fn print_resources( archetypes : &Archetypes, components : &Components )
{
  let mut r : Vec<String> = archetypes
  .resource()
  .components()
  .map(|id| components.get_info(id).unwrap())
  // get_short_name removes the path information
  // i.e. `bevy_audio::audio::Audio` -> `Audio`
  // if you want to see the path info replace
  // `TypeRegistration::get_short_name` with `String::from`
  .map(|info| TypeRegistration::get_short_name(info.name()))
  .collect();

  // sort list alphebetically
  r.sort();
  r.iter().for_each(|name| println!("{}", name));
}

///
/// The system iterates resize events and print them.
///

pub fn on_resize_system(mut resize_reader : EventReader<WindowResized>)
{
  for e in resize_reader.iter()
  {
    println!( "event {:?}", e );
  }
}

///
/// Main
///

fn main()
{
  let mut app = App::new();

  app
  .add_plugins( DefaultPlugins )
  // .add_plugin( bevy::diagnostic::LogDiagnosticsPlugin::default() )
  .add_plugin( bevy::diagnostic::FrameTimeDiagnosticsPlugin::default() )
  .add_startup_system( setup.system() )
  .add_system( ball_move.system() )
  .add_system( ball_move.system() )
  .add_system( bevy::input::system::exit_on_esc_system )
  ;

  // app.add_plugin( bevy_inspector_egui::WorldInspectorPlugin::new() );

  // bevy_mod_debugdump::print_schedule( &mut app );
  // bevy_mod_debugdump::print_render_graph( &mut app );
  // app.add_system( print_render_graph );

  app.run();
}
