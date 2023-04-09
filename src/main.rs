use bevy::{
    prelude::*,
  };
  
  mod splash;
  mod menu;
  mod game; 
  mod tower; 
  mod enemy;
  mod bullet;
  mod base;
  mod season;
  mod map;
  
  #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
  enum GameState {
      #[default]
      Splash,
      Menu,
      Game,
      Pause,
      GameLost,
      GameWon
  }
  
  #[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
  enum DisplayQuality {
      Low,
      Medium,
      High,
  }
  
  #[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
  struct Volume(u32);
  
  
  const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
  
  
  fn main() {
    App::new()
      .add_plugins(DefaultPlugins
        .set(WindowPlugin {primary_window: Some(Window {
            title: String::from("Rusty Towers"),
            resizable: false,
            ..Default::default()
        }),
        ..default()
        }).set(ImagePlugin::default_nearest()))
      .insert_resource(DisplayQuality::Medium)
      .insert_resource(Volume(7))
      .add_startup_system(setup)
      .add_state::<GameState>()
      .add_plugin(splash::SplashPlugin)
      .add_plugin(menu::MenuPlugin)
      .add_plugin(game::GamePlugin)
      .run();
  }
  
  fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
  }
  
  
  // Generic system that takes a component as a parameter, and will despawn all entities with that component
  fn despawn_with_component<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
  }