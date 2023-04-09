use std::f32::consts::E;
use std::time::Duration;

use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    input::{mouse::{MouseButtonInput, MouseMotion, MouseWheel}, keyboard::KeyboardInput}, 
    window::PrimaryWindow,
    text::{BreakLineOn, Text2dBounds},
};

use super::GameState;

use crate::{tower::{TowerBundle, TowerStats, TOWER_RADIUS, TOWER_COLOR, TowerPlugin}, base::BASE_COLOR, game, despawn_with_component, bullet::Bullet, enemy::EnemyStats, season::SeasonSchedule};
use crate::enemy::{EnemyPlugin, WaveTimer, ENEMY_SPAWN_INTERVAL_SECONDS};
use crate::bullet::BulletPlugin;
use crate::base::{Base, BASE_RADIUS, BASE_INITIAL_HEALTH};
use crate::season::{SeasonPlugin, SeasonBarPart};
use crate::map::MapPlugin;
use crate::map::{Map, Wall};

#[derive(Component)]
struct AnimateTranslation;

#[derive(Component)]
struct EndGameText;

pub struct GamePlugin;

pub const HEAL_AMOUNT: f32 = 100.0;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(startup.in_schedule(OnEnter(GameState::Game)))
        .add_plugin(TowerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(SeasonPlugin)
        .add_plugin(MapPlugin)
        .insert_resource(WaveTimer {
            // create the repeating timer
            timer: Timer::new(Duration::from_secs(ENEMY_SPAWN_INTERVAL_SECONDS as u64), TimerMode::Repeating),
            force_wave: false
        })
        .add_system(animate_translation)
        .add_system(sync_base_size)
        .add_system(end_game.in_schedule(OnEnter(GameState::GameWon)))
        .add_system(end_game.in_schedule(OnEnter(GameState::GameLost)))
        .add_system(
            despawn_with_component::<TowerStats>.in_schedule(OnEnter(GameState::Menu)),
        )
        .add_system(
            despawn_with_component::<Bullet>.in_schedule(OnEnter(GameState::Menu)),
        )
        .add_system(
            despawn_with_component::<EnemyStats>.in_schedule(OnEnter(GameState::Menu)),
        )
        .add_system(
            despawn_with_component::<Base>.in_schedule(OnEnter(GameState::Menu)),
        )
        .add_system(
            despawn_with_component::<Map>.in_schedule(OnEnter(GameState::Menu)),
        )
        .add_system(
            despawn_with_component::<Wall>.in_schedule(OnEnter(GameState::Menu)),
        )
        .add_system(
            despawn_with_component::<SeasonBarPart>.in_schedule(OnEnter(GameState::Menu)),
        )
        .add_system(
            despawn_with_component::<EndGameText>.in_schedule(OnEnter(GameState::Menu)),
        )
        .add_system(listen_for_restart.run_if(in_state(GameState::GameLost)))
        .add_system(listen_for_restart.run_if(in_state(GameState::GameWon)));
    }
}

fn compute_scale(health: f32) -> f32 {
    (health / BASE_INITIAL_HEALTH).max(0.25)
}

fn sync_base_size(mut base_query: Query<(&Base, &mut Transform)>) {
    for (base, mut base_transform) in base_query.iter_mut() {
        base_transform.scale = Vec3::splat(compute_scale(base.health));
    }
}

fn startup(mut commands: Commands,    
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>) {

    commands.spawn((Base {health: BASE_INITIAL_HEALTH}, 
    MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(BASE_RADIUS).into()).into(),
        material: materials.add(ColorMaterial::from(BASE_COLOR)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    }
    ));
}


fn end_game(mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    game_state: Res<State<GameState>>,
    base_query: Query<&Base> )
    {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::Center;

    let base_health = base_query.get_single().unwrap().health as i32;

    let text = match game_state.0 {
        GameState::GameWon => format!("You win! Score: {}\nPress any key to return to the menu.", base_health),
        GameState::GameLost => "Game Over! You lost.\nPress any key to return to the menu.".to_string(),
        _ => "unreachable".to_string()
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(text, text_style.clone())
                .with_alignment(text_alignment),
            transform: Transform::from_translation(Vec3::new(0., 0., 6.)),
            ..default()
        },
        EndGameText
    ));
}

fn listen_for_restart(mut commands: Commands, 
    mut wave_timer: ResMut<WaveTimer>,
    mut key_evr: EventReader<KeyboardInput>, 
    mut game_state: ResMut<NextState<GameState>>,
    query: Query<Entity, With<EndGameText>>, 
    mut season_schedule: ResMut<SeasonSchedule>) {
    for ev in key_evr.iter() {
        match ev.state {
            _ => {
                for entity in query.iter() {
                    commands.entity(entity).despawn();
                }
                game_state.set(GameState::Menu);
                wave_timer.timer.reset();
            }, 
        }
    }
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