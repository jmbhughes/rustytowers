use bevy::{prelude::*,
            window::PrimaryWindow,
            sprite::MaterialMesh2dBundle,
};
use std::time::Duration;

use super::GameState;

pub struct SeasonPlugin;

impl Plugin for SeasonPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_season_bar.run_if(in_state(GameState::Game)))
           .add_system(initialize_season_bar.in_schedule(OnEnter(GameState::Game)))
           .add_state::<Season>()
           .insert_resource(ElapsedCounter {seconds_elapsed: 0., pixels_per_second: 0.})
           .insert_resource(SeasonSchedule {
            intervals: vec![SeasonInterval{season: Season::Build, duration: 10.}, 
                            SeasonInterval{season: Season::Heal, duration: 5.},
                            SeasonInterval{season: Season::Build, duration: 15.},
                            SeasonInterval{season: Season::Heal, duration: 3.},
                            SeasonInterval{season: Season::Build, duration: 10.}],
            current_season_index: 0,
            current_season_timer: Timer::new(Duration::from_secs(10. as u64), TimerMode::Once)
           });
    }
}


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum Season {
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

#[derive(Resource)]
pub struct SeasonSchedule {
    intervals: Vec<SeasonInterval>,
    current_season_timer: Timer,
    current_season_index: usize
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
    mut season_schedule: ResMut<SeasonSchedule>,
    mut current_season: ResMut<NextState<Season>>,
) {
    info!("initialize season bar");

    season_schedule.current_season_index = 0;
    let next_interval = season_schedule.intervals.get(season_schedule.current_season_index).unwrap();
    current_season.set(next_interval.season);
    season_schedule.current_season_timer = Timer::new(Duration::from_secs(next_interval.duration as u64), TimerMode::Once);

    let Ok(window) = primary_window_query.get_single() else {
        panic!("no window!");
    };

    let total_duration: f32 = season_schedule.intervals.iter()
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

    for interval in season_schedule.intervals.iter() {
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
    mut season_time_indicator_query: Query<(&SeasonBarTimeIndicator, &mut Transform)>,
    mut season_schedule: ResMut<SeasonSchedule>,
    mut current_season: ResMut<NextState<Season>>,
    mut game_state: ResMut<NextState<GameState>>
) {
    let delta = time.delta_seconds();

    season_schedule.current_season_timer.tick(time.delta());
    if season_schedule.current_season_timer.finished() {
        season_schedule.current_season_index += 1;
        if season_schedule.current_season_index < season_schedule.intervals.len() {
            let next_interval = season_schedule.intervals.get(season_schedule.current_season_index).unwrap();
            current_season.set(next_interval.season);
            season_schedule.current_season_timer = Timer::new(Duration::from_secs(next_interval.duration as u64), TimerMode::Once);
            info!("season changed");
        } else {
            info!("THE GAME HAS BEEN WON!");
            game_state.set(GameState::GameWon);
        }
    }

    let Ok((_, mut transform)) = season_time_indicator_query.get_single_mut() else {
        info!("no bar indicator!");
        return;
    };

    transform.translation.x += delta * elapsed_counter.pixels_per_second;
    
}