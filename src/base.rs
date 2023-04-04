use bevy::prelude::*;


pub const BASE_COLOR: Color = Color::DARK_GREEN;
pub const BASE_RADIUS: f32 = 30.;
pub const BASE_INITIAL_HEALTH: f32 = 1000.;

#[derive(Component)]
pub struct Base {
    pub health: f32
}