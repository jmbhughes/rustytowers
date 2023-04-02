use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel}, window::PrimaryWindow
};
use crate::enemy::EnemyStats;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_bullets);
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


pub fn move_bullets(
    time: Res<Time>, 
    mut bullet_query: Query<(&Bullet, &mut Transform)>, 
    mut enemy_query: Query<(Entity, &EnemyStats, &Transform), Without<Bullet>>) {

    for (bullet, mut transform) in bullet_query.iter_mut() {
        if let Ok((_, target_stats, target_transform)) = enemy_query.get(bullet.target) {
            let dist = transform
            .translation
            .truncate()
            .distance(target_transform.translation.truncate());

            let delta = time.delta_seconds();
            let step = bullet.speed * delta;
            transform.translation.x +=
                    step / dist * (target_transform.translation.x - transform.translation.x);
            transform.translation.y +=
                    step / dist * (target_transform.translation.y - transform.translation.y);

        } else {

        }
    }
}

// pub fn spawn_bullet(
//     x: f32,
//     y: f32,
//     target: Entity,
//     damage: u32,
//     speed: f32,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     commands: &mut Commands
// ) {
//     commands.spawn((
//         MaterialMesh2dBundle {
//             mesh: meshes.add(shape::Circle::new(BULLET_RADIUS).into()).into(),
//             material: materials.add(ColorMaterial::from(BULLET_COLOR)),
//             transform: Transform::from_xyz(x, y, 0.),
//             ..default()
//         },
//         Bullet {
//             target,
//             damage,
//             speed
//         },
//     ));
// }