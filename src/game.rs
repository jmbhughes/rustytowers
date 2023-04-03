use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel}, window::PrimaryWindow,
};

use super::GameState;

use crate::{tower::{TowerBundle, TowerStats, TOWER_RADIUS, TOWER_COLOR, TowerPlugin}, base::BASE_COLOR};
use crate::enemy::EnemyPlugin;
use crate::bullet::BulletPlugin;
use crate::base::Base;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(startup.in_schedule(OnEnter(GameState::Game)))
        .add_plugin(TowerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(BulletPlugin);
    }
}

fn startup(mut commands: Commands,    
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>) {

    let base_radius = 30.;
    commands.spawn((Base, 
    MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(base_radius).into()).into(),
        material: materials.add(ColorMaterial::from(BASE_COLOR)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    }
    ));
}
