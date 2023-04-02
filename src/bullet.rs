use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel}, window::PrimaryWindow
};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {

    }
}

#[derive(Component)]
pub struct Bullet {
    pub target: Entity,
    pub damage: u32,
    pub speed: f32
}

pub const BULLET_RADIUS: f32 = 3.;
pub const BULLET_COLOR: Color = Color::RED;


pub fn spawn_bullet(
    x: f32,
    y: f32,
    target: Entity,
    damage: u32,
    speed: f32,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    commands: &mut Commands
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(BULLET_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(BULLET_COLOR)),
            transform: Transform::from_xyz(x, y, 0.),
            ..default()
        },
        Bullet {
            target,
            damage,
            speed
        },
    ));
}