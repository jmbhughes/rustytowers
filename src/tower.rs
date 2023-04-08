use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel}, window::PrimaryWindow
};

use crate::{enemy::{EnemyStats, WaveTimer, ENEMY_SPAWN_INTERVAL_SECONDS}, base::BASE_RADIUS};
use crate::bullet::{BULLET_COLOR, BULLET_RADIUS, Bullet};
use super::GameState;
use crate::game::{fall_off_damage_curve, euclidean_distance, HEAL_AMOUNT};
use crate::base::Base;
use crate::season::Season;

pub struct TowerPlugin;

pub const TOWER_RADIUS: f32 = 25.;
pub const TOWER_COLOR: Color = Color::PURPLE;
pub const TOWER_INITIAL_HEALTH: f32 = 100.;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(place_tower)
        .add_system(sync_size)
        .add_system(shoot_enemies)
        .add_system(heal_tower_and_base);
    }
}

#[derive(Component, Default, Debug)]
pub struct TowerStats {
    pub x: f32,
    pub y: f32,
    pub level: u32,
    pub range: f32,
    pub damage: f32,
    pub upgrade_price: u32,
    pub speed: f32,
    pub health: f32
}

#[derive(Component, Default)]
pub struct TowerState {
    pub timer: Timer,
}


#[derive(Bundle, Default)]
pub struct TowerBundle {
    pub stats: TowerStats,
    pub state: TowerState,
}

impl TowerBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            stats: TowerStats {
                x: x,
                y: y,
                level: 1,
                range: 128.0,
                damage: 50.,
                upgrade_price: 10,
                speed: 500.0,
                health: TOWER_INITIAL_HEALTH
            },
            state: TowerState {
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            },
            ..Default::default()
        }
    }
}

fn shoot_enemies(
    mut commands: Commands, 
    time: Res<Time>, 
    mut tower_query: Query<(&TowerStats, &mut TowerState)>, 
    enemy_query: Query<(Entity, &Transform), With<EnemyStats>>, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>> ) {

    for (tower_stat, mut tower_state) in tower_query.iter_mut() {
        // only fire if the tower is not on cooldown
        tower_state.timer.tick(time.delta());
        if !tower_state.timer.finished() {
            continue;
        }

        // find the closest enemy
        let closest_result = enemy_query.iter()
            .map(|(enemy, transform)| (enemy, ((transform.translation.x - tower_stat.x).powi(2) + (transform.translation.y - tower_stat.y).powi(2)).sqrt()))
            .min_by(|(enemy1, distance1), (enemy2, distance2)| distance1.total_cmp(distance2));

        // if there was a closest enemy, fire a bullet if they're within range
        if let Some((closest_enemy, closest_distance)) = closest_result {
            if closest_distance < tower_stat.range {
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(BULLET_RADIUS).into()).into(),
                        material: materials.add(ColorMaterial::from(BULLET_COLOR)),
                        transform: Transform::from_xyz(tower_stat.x, tower_stat.y, 0.),
                        ..default()
                    },
                    Bullet {
                        target: closest_enemy,
                        damage: tower_stat.damage,
                        speed: tower_stat.speed
                    },
                ));
            }
        }
    }
}

fn place_tower(
    mut commands: Commands, 
    mouse_button_input: Res<Input<MouseButton>>, 
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    current_season: Res<State<Season>>,
    game_state: Res<State<GameState>>,
    mut update_game_state: ResMut<NextState<GameState>>,
    mut other_towers_query: Query<(Entity, &mut TowerStats, &Transform)>,
    mut enemies_query: Query<(Entity, &mut EnemyStats, &Transform)>,
    mut base_query: Query<(&mut Base, &Transform)>,
) {

    if game_state.0 == GameState::Game && current_season.0 == Season::Build {

        let Ok(window) = primary_window_query.get_single() else {
                return;
        };

        if let Some(_position) = window.cursor_position() {
            if mouse_button_input.just_pressed(MouseButton::Left) {
                let x = _position.x - window.width() / 2.0;
                let y = _position.y - window.height() / 2.0;

                                
                for (tower_entity, mut tower_stat, tower_transform) in other_towers_query.iter_mut() {
                    let distance = euclidean_distance(x, y, tower_transform.translation.x, tower_transform.translation.y);
                    let damage = fall_off_damage_curve(distance, 100., 10., 4.);
                    if tower_stat.health >= damage {
                        info!("{} results in {} to {}", damage, tower_stat.health, tower_stat.health - damage);
                        tower_stat.health -= damage;
                    } else {
                        info!("tower despawned");
                        commands.entity(tower_entity).despawn();
                    }
                }

                for (enemy_entity, mut enemy_stat, enemy_transform) in enemies_query.iter_mut() {
                    let distance = euclidean_distance(x, y, enemy_transform.translation.x, enemy_transform.translation.y);
                    let damage = fall_off_damage_curve(distance, 100., 10., 4.);
                    if enemy_stat.health >= damage {
                        info!("enemy {} results in {} to {}", damage, enemy_stat.health, enemy_stat.health - damage);
                        enemy_stat.health -= damage;
                    } else {
                        info!("enemy despawned");
                        commands.entity(enemy_entity).despawn();
                    }
                }

                for (mut base, base_transform) in base_query.iter_mut() {
                    let distance = euclidean_distance(x, y, base_transform.translation.x, base_transform.translation.y);
                    let damage = fall_off_damage_curve(distance, 100., 10., 4.);
                    if base.health >= damage {
                        info!("base {} results in {} to {}", damage, base.health, base.health - damage);
                        base.health -= damage;
                    } else {
                        info!("base destroyed");
                        update_game_state.set(GameState::GameLost);
                    }
                }

                commands.spawn((
                    TowerBundle::new(x, y),
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(TOWER_RADIUS).into()).into(),
                        material: materials.add(ColorMaterial::from(TOWER_COLOR)),
                        transform: Transform::from_xyz(x, y, 0.),
                        ..default()
                }));

                
            }   
    }
    }
}

fn heal_tower_and_base(
    mut commands: Commands, 
    mouse_button_input: Res<Input<MouseButton>>, 
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    current_season: Res<State<Season>>,
    game_state: Res<State<GameState>>,
    mut towers_query: Query<(&mut TowerStats, &Transform)>,
    mut base_query: Query<(&mut Base, &Transform)>,
    mut wave_timer: ResMut<WaveTimer>
) {

    if game_state.0 == GameState::Game && current_season.0 == Season::Heal {

        let Ok(window) = primary_window_query.get_single() else {
                return;
        };

        if let Some(_position) = window.cursor_position() {
            if mouse_button_input.just_pressed(MouseButton::Left) {
                let x = _position.x - window.width() / 2.0;
                let y = _position.y - window.height() / 2.0;

                                
                for (mut tower_stat, tower_transform) in towers_query.iter_mut() {
                    let distance = euclidean_distance(x, y, tower_transform.translation.x, tower_transform.translation.y);
                    if distance < TOWER_RADIUS * tower_transform.scale.x{
                        tower_stat.health += HEAL_AMOUNT;
                        wave_timer.force_wave = true;
                    }
                }

                for (mut base, base_transform) in base_query.iter_mut() {
                    let distance = euclidean_distance(x, y, base_transform.translation.x, base_transform.translation.y);
                    if distance < BASE_RADIUS * base_transform.scale.x {
                        base.health += HEAL_AMOUNT;
                        wave_timer.force_wave = true;
                    }
                }
                
            }   
    }
    }
}



fn compute_scale(health: f32) -> f32 {
    (health / TOWER_INITIAL_HEALTH).max(0.25)
}

fn sync_size(mut tower_query: Query<(&TowerStats, &mut Transform)>) {
    for (tower_stat, mut tower_transform) in tower_query.iter_mut() {
        tower_transform.scale = Vec3::splat(compute_scale(tower_stat.health));
    }
}