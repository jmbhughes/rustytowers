use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};

use super::{despawn_screen, DisplayQuality, GameState, Volume, TEXT_COLOR};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            setup_game.in_schedule(OnEnter(GameState::Game)),
        ));
    }
}

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
    
    //commands.spawn(Camera2dBundle::default());
    
    }