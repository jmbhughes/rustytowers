use bevy::prelude::*;


pub const BASE_COLOR: Color = Color::DARK_GREEN;
pub const BASE_RADIUS: f32 = 30.;

#[derive(Component)]
pub struct Base {
    pub health: u32
}