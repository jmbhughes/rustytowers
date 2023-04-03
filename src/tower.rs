use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel}, window::PrimaryWindow
};

use crate::enemy::EnemyStats;
use crate::bullet::{BULLET_COLOR, BULLET_RADIUS, Bullet};
use super::GameState;

pub struct TowerPlugin;

pub const TOWER_RADIUS: f32 = 25.;
pub const TOWER_COLOR: Color = Color::PURPLE;


impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(place_tower)
        .add_system(shoot_enemies);
    }
}

#[derive(Component, Default, Debug)]
pub struct TowerStats {
    pub x: f32,
    pub y: f32,
    pub level: u32,
    pub range: f32,
    pub damage: u32,
    pub upgrade_price: u32,
    pub speed: f32,
    pub health_percent: u32
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
                damage: 1,
                upgrade_price: 10,
                speed: 1.0,
                health_percent: 100
            },
            state: TowerState {
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            },
            ..Default::default()
        }
    }
}

fn shoot_enemies(
    mut commands: Commands, 
    time: Res<Time>, 
    mut tower_query: Query<(&TowerStats, &mut TowerState)>, 
    enemy_query: Query<(Entity, &EnemyStats), With<EnemyStats>>, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>> ) {

    for (tower_stat, mut tower_state) in tower_query.iter_mut() {
        tower_state.timer.tick(time.delta());
        if !tower_state.timer.finished() {
            continue;
        }

        for (enemy, enemy_stat) in enemy_query.iter() {
            if ((enemy_stat.x - tower_stat.x).powi(2) + (enemy_stat.y - tower_stat.y).powi(2)).sqrt() < tower_stat.range {
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(BULLET_RADIUS).into()).into(),
                        material: materials.add(ColorMaterial::from(BULLET_COLOR)),
                        transform: Transform::from_xyz(tower_stat.x, tower_stat.y, 0.),
                        ..default()
                    },
                    Bullet {
                        target: enemy,
                        damage: 50,
                        speed: 200.
                    },
                ));
                break;
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
    game_state: Res<State<GameState>>) {

    if game_state.0 == GameState::Game {

        let Ok(window) = primary_window_query.get_single() else {
                return;
        };

        if let Some(_position) = window.cursor_position() {
            if mouse_button_input.just_released(MouseButton::Left) {
                info!("left mouse just released");
                info!("{} {}", _position.x, _position.y);
                let x = _position.x - window.width() / 2.0;
                let y = _position.y - window.height() / 2.0;
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