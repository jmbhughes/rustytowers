use std::f32::consts::E;
use std::time::Duration;

use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel}, window::PrimaryWindow,
    text::{BreakLineOn, Text2dBounds},
};

use super::GameState;

use crate::{tower::{TowerBundle, TowerStats, TOWER_RADIUS, TOWER_COLOR, TowerPlugin}, base::BASE_COLOR};
use crate::enemy::{EnemyPlugin, WaveTimer, ENEMY_SPAWN_INTERVAL_SECONDS};
use crate::bullet::BulletPlugin;
use crate::base::{Base, BASE_RADIUS};



#[derive(Component)]
struct AnimateTranslation;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(startup.in_schedule(OnEnter(GameState::Game)))
        .add_plugin(TowerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(BulletPlugin)
        .add_system(animate_translation)
        .add_system(end_game.in_schedule(OnEnter(GameState::GameEnd)));
    }
}

fn startup(mut commands: Commands,    
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>) {

    commands.spawn((Base {health: 1000.}, 
    MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(BASE_RADIUS).into()).into(),
        material: materials.add(ColorMaterial::from(BASE_COLOR)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    }
    ));

    commands.insert_resource(WaveTimer {
        // create the repeating timer
        timer: Timer::new(Duration::from_secs(ENEMY_SPAWN_INTERVAL_SECONDS as u64), TimerMode::Repeating),
    });
}

fn end_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::Center;
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Game Over!", text_style.clone())
                .with_alignment(text_alignment),
            ..default()
        },
        AnimateTranslation,
    ));
}

fn animate_translation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateTranslation>)>,
) {
    for mut transform in &mut query {
        transform.translation.x = 100.0 * time.elapsed_seconds().sin() - 400.0;
        transform.translation.y = 100.0 * time.elapsed_seconds().cos();
    }
}

pub fn fall_off_damage_curve(distance: f32, base_damage: f32, min_distance: f32, k: f32) -> f32{
    if distance < min_distance {
        base_damage
    } else {
        base_damage / (1. + E.powf(-k) * (distance-min_distance))
    }
}

pub fn euclidean_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}