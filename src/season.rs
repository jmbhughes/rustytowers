use bevy::{prelude::*,
            window::PrimaryWindow,
            sprite::MaterialMesh2dBundle,
};

use super::GameState;

pub struct SeasonPlugin;

impl Plugin for SeasonPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_season_bar.run_if(in_state(GameState::Game)))
           .add_system(initialize_season_bar.in_schedule(OnEnter(GameState::Game)))
           .add_state::<Season>()
           .insert_resource(ElapsedCounter {seconds_elapsed: 0., pixels_per_second: 0.});
    }
}


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
  enum Season {
      #[default]
      Build,
      Heal,
      Upgrade,
      Neutralize
  }


fn season_color(season: Season) -> Color {
    match season {
        Season::Build => Color::GREEN,
        Season::Heal => Color::RED,
        Season::Upgrade => Color::GOLD,
        Season::Neutralize => Color::VIOLET
    }
}

#[derive(Resource)]
pub struct ElapsedCounter {
    pub seconds_elapsed: f32,
    pub pixels_per_second: f32
}

struct SeasonInterval {
    season : Season,
    duration : f32
}

pub const SEASON_BAR_HEIGHT: f32 = 30.;

#[derive(Component)]
pub struct SeasonBarTimeIndicator;

#[derive(Component)]
pub struct SeasonBarPart;

fn initialize_season_bar(mut commands: Commands, 
    mut elapsed_counter: ResMut<ElapsedCounter>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
) {
    info!("initialize season bar");

    let season_data = vec![SeasonInterval{season: Season::Build, duration: 120.}, 
                                                SeasonInterval{season: Season::Heal, duration: 30.}];

    let Ok(window) = primary_window_query.get_single() else {
        panic!("no window!");
    };

    let total_duration: f32 = season_data.iter()
        .map(|interval| interval.duration)
        .sum();

    elapsed_counter.seconds_elapsed = 0.;
    elapsed_counter.pixels_per_second = window.width() / total_duration;


    let bar_y = window.height()/2. - SEASON_BAR_HEIGHT;
    let mut accumulated_shift = -window.width() / 2.;

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(10., SEASON_BAR_HEIGHT)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(accumulated_shift+5., bar_y, 1.)),
        ..default()
    }, SeasonBarTimeIndicator, SeasonBarPart));

    for interval in season_data.iter() {
        let width = interval.duration / total_duration * window.width();
        commands.spawn((SpriteBundle {
            sprite: Sprite {
                color: season_color(interval.season),
                custom_size: Some(Vec2::new(width, SEASON_BAR_HEIGHT)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(accumulated_shift+width/2., bar_y, 0.)),
            ..default()
        }, SeasonBarPart));

        accumulated_shift += width;
    }
    
}


fn update_season_bar(time: Res<Time>,
    mut elapsed_counter: ResMut<ElapsedCounter>, 
    mut season_time_indicator_query: Query<(&SeasonBarTimeIndicator, &mut Transform)>) {
    let delta = time.delta_seconds();

    let Ok((_, mut transform)) = season_time_indicator_query.get_single_mut() else {
        info!("no bar indicator!");
        return;
    };

    transform.translation.x += delta * elapsed_counter.pixels_per_second;
}