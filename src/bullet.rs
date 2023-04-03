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
    pub damage: f32,
    pub speed: f32
}

pub const BULLET_RADIUS: f32 = 3.;
pub const BULLET_COLOR: Color = Color::RED;


pub fn move_bullets(
    mut commands: Commands,
    time: Res<Time>, 
    mut bullet_query: Query<(Entity, &Bullet, &mut Transform)>, 
    mut enemy_query: Query<(Entity, &mut EnemyStats, &Transform), Without<Bullet>>) {

    for (bullet_entity, bullet, mut transform) in bullet_query.iter_mut() {
        if let Ok((target_entity, mut target_stats, target_transform)) = enemy_query.get_mut(bullet.target) {
            let dist = transform
            .translation
            .truncate()
            .distance(target_transform.translation.truncate());

            if dist > BULLET_RADIUS {
                let delta = time.delta_seconds();
                let step = bullet.speed * delta;
                transform.translation.x +=
                        step / dist * (target_transform.translation.x - transform.translation.x);
                transform.translation.y +=
                        step / dist * (target_transform.translation.y - transform.translation.y);
            } else {
                if target_stats.health >= bullet.damage {
                    target_stats.health -= bullet.damage;
                }
                commands.entity(bullet_entity).despawn();
                if target_stats.health <= 0. {
                    commands.entity(target_entity).despawn();
                }
            }

        } else {
            commands.entity(bullet_entity).despawn();
        }
    }
}
