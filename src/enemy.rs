use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel}, window::PrimaryWindow
};
use super::GameState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(place_enemy);
    }
}


pub const ENEMY_RADIUS: f32 = 5.;
pub const ENEMY_COLOR: Color = Color::BLACK;

#[derive(Component, Default)]
pub struct EnemyState {
    pub timer: Timer,
}

#[derive(Component, Default)]
pub struct EnemyStats {
    health: u8,
    x: f32, 
    y: f32
}

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    pub stats: EnemyStats,
    pub state: EnemyState
}

impl EnemyBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            stats: EnemyStats {
                health: 100,
                x: x,
                y: y
            },
            state: EnemyState {
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            },
            ..Default::default()
        }
    }
}

fn place_enemy(mut commands: Commands, 
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
            if mouse_button_input.just_released(MouseButton::Right) {
                info!("right mouse just released");
                info!("{} {}", _position.x, _position.y);
                let x = _position.x - window.width() / 2.0;
                let y = _position.y - window.height() / 2.0;
                commands.spawn((
                    EnemyBundle::new(x, y),
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(ENEMY_RADIUS).into()).into(),
                        material: materials.add(ColorMaterial::from(ENEMY_COLOR)),
                        transform: Transform::from_xyz(x, y, 0.),
                        ..default()
                }));
            }   
    }
    }
}