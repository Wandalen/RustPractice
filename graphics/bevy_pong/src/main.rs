use bevy::prelude::*;

//

#[derive(Component)]
pub struct Movable
{
  pub speed : f32,
  pub direction : Vec3,
}

//

fn setup( mut commands : Commands )
{

  // 2D orthographic camera
  let camera = OrthographicCameraBundle::new_2d();

  commands.spawn_bundle( camera );

  // commands.spawn( Camera2DComponents::default() );
  ball_spawn( &mut commands );

}

//

fn ball_spawn( commands : &mut Commands )
{

  let size = 25.0;
  let sprite = Sprite { custom_size : Some( Vec2::new( size, size ) ), .. Default::default() };
  commands.spawn_bundle( SpriteBundle
  {
    sprite,
    .. Default::default()
  })
  .insert( Movable { speed : 1.0, direction : Vec3::new( 1.0, 1.0, 0.0 ).normalize() } )
  ;

}

//

fn ball_move( mut query : Query<(&Movable, &mut Transform)> )
{

  let (movable, mut transform) = query.single_mut();
  transform.translation += movable.direction * movable.speed;

}

//

fn main()
{

  let mut app = App::new();

  app
  .add_plugins( DefaultPlugins )
  // .add_plugin( bevy::diagnostic::LogDiagnosticsPlugin::default() )
  .add_plugin( bevy::diagnostic::FrameTimeDiagnosticsPlugin::default() )
  .add_startup_system( setup.system() )
  .add_system( ball_move.system() )
  .add_system( bevy::input::system::exit_on_esc_system )
  ;

  // app.add_plugin( bevy_inspector_egui::WorldInspectorPlugin::new() );

  // bevy_mod_debugdump::print_schedule( &mut app );
  // bevy_mod_debugdump::print_render_graph( &mut app );
  // app.add_system( print_render_graph );

  app.run();
}
