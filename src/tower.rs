use bevy::prelude::*;
pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

#[derive(Component, Default, Debug)]
pub struct TowerStats {
    pub x: f32,
    pub y: f32,
    pub level: u32,
    pub range: f32,
    pub damage: u32,
    pub upgrade_price: u32,
    pub speed: f32,
    pub health_percent: u32
}

#[derive(Component, Default)]
pub struct TowerState {
    pub timer: Timer,
}


#[derive(Bundle, Default)]
pub struct TowerBundle {
    pub stats: TowerStats,
    pub state: TowerState,
}

impl TowerBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            stats: TowerStats {
                x: x,
                y: y,
                level: 1,
                range: 128.0,
                damage: 1,
                upgrade_price: 10,
                speed: 1.0,
                health_percent: 100
            },
            state: TowerState {
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            },
            ..Default::default()
        }
    }
}

fn shoot_enemies() {

}

