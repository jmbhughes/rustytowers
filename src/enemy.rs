use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel}, window::PrimaryWindow
};
use super::GameState;
use crate::{base::{Base, BASE_RADIUS}, game};
use rand::Rng;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_enemy.run_if(in_state(GameState::Game)))
           .add_system(move_enemy.run_if(in_state(GameState::Game)))
           .add_system(enemy_damage_base.run_if(in_state(GameState::Game)));    }
}


pub const ENEMY_RADIUS: f32 = 5.;
pub const ENEMY_COLOR: Color = Color::BLACK;
pub const ENEMY_SPAWN_INTERVAL_SECONDS: u32 = 3;
pub const ENEMY_SPAWN_PER_INTERVAL: u32 = 25;


#[derive(Resource)]
pub struct WaveTimer {
    pub timer: Timer,
    pub force_wave: bool
}

#[derive(Component, Default)]
pub struct EnemyState {
    pub timer: Timer,
}

#[derive(Component, Default)]
pub struct EnemyStats {
    pub health: f32,
    pub destination: Vec2,
    pub speed: f32,
    pub damage: f32
}

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    pub stats: EnemyStats,
    pub state: EnemyState
}

impl EnemyBundle {
    pub fn new(x: f32, y: f32, destination: Vec2) -> Self {
        Self {
            stats: EnemyStats {
                health: 100.,
                destination: destination,
                speed: 50.,
                damage: 100.
            },
            state: EnemyState {
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            },
            ..Default::default()
        }
    }
}

fn move_enemy(
    time: Res<Time>, 
    mut enemy_query: Query<(&EnemyStats, &mut Transform)>) {
        for (enemy_stat, mut transform) in enemy_query.iter_mut() {
            let dist = transform
            .translation
            .truncate()
            .distance(enemy_stat.destination);

            let delta = time.delta_seconds();
            let step = enemy_stat.speed * delta;
            transform.translation.x +=
                    step / dist * (enemy_stat.destination[0] - transform.translation.x);
            transform.translation.y +=
                    step / dist * (enemy_stat.destination[1] - transform.translation.y);
        }
}

fn enemy_damage_base(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &EnemyStats, &Transform)>,
    mut base_query: Query<(Entity, &mut Base, &Transform)>,
    mut game_state: ResMut<NextState<GameState>>
) {

    let Ok((base_entity, mut base, base_transform)) = base_query.get_single_mut() else {
        info!("no base!");
        return;
    };

    for (enemy_entity, enemy_stat, enemy_transform) in enemy_query.iter() {
        if ((enemy_transform.translation.x - base_transform.translation.x).powi(2) + (enemy_transform.translation.y - base_transform.translation.y).powi(2)).sqrt() < BASE_RADIUS * base_transform.scale.x {
            base.health -= enemy_stat.damage;
            commands.entity(enemy_entity).despawn();
            if base.health <= 0. {
                game_state.set(GameState::GameLost);
            }
        }
    }

}

fn spawn_enemy(mut commands: Commands, 
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_state: Res<State<GameState>>,
    base_query: Query<(&Base, &Transform)>,
    time: Res<Time>,
    mut wave_timer: ResMut<WaveTimer>
) {
    if game_state.0 == GameState::Game {

        let Ok(window) = primary_window_query.get_single() else {
                panic!("no window!");
        };

        let Ok((base, base_transform)) = base_query.get_single() else {
            panic!("no base!");
        };

        wave_timer.timer.tick(time.delta());

        if wave_timer.timer.finished() || wave_timer.force_wave {
            let mut rng = rand::thread_rng();

            for _ in 0..ENEMY_SPAWN_PER_INTERVAL {
                let x = rng.gen_range((-window.width() / 2.)..(window.width() / 2.));
                let y = rng.gen_range((-window.height() / 2.)..(window.height() / 2.));

                commands.spawn((
                    EnemyBundle::new(x, y, base_transform.translation.truncate()),
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(ENEMY_RADIUS).into()).into(),
                        material: materials.add(ColorMaterial::from(ENEMY_COLOR)),
                        transform: Transform::from_xyz(x, y, 0.),
                        ..default()
                    }));
            }
            wave_timer.force_wave = false;
        }
    }
}
