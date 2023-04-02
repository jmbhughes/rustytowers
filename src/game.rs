use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel}, window::PrimaryWindow,
};

use super::GameState;

use crate::tower::{TowerBundle, TowerStats, TOWER_RADIUS, TOWER_COLOR, TowerPlugin};
use crate::enemy::EnemyPlugin;
use crate::bullet::BulletPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(startup.in_schedule(OnEnter(GameState::Game)))
        .add_plugin(TowerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(BulletPlugin);
        //.add_system(show_towers)
    }
}

fn startup(mut commands: Commands,    
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(TOWER_RADIUS).into()).into(),
        material: materials.add(ColorMaterial::from(TOWER_COLOR)),
        transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
        ..default()
    });
}
