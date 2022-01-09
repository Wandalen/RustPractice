use bevy::prelude::*;

use bevy::core::CorePlugin;
use bevy::input::InputPlugin;
use bevy::window::WindowPlugin;
use bevy::ui::UiPlugin;
use bevy::render::RenderPlugin;

fn main()
{
  App::build()
  // .add_default_stages()
  // .add_default_plugins()
  .add_plugin( CorePlugin::default() )
  .add_plugin( InputPlugin::default() )
  .add_plugin( WindowPlugin::default() )
  // .add_plugin( RenderPlugin::default() )
  .add_plugin( UiPlugin::default() )
  .run()
  ;
}
