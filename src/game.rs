use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel}, window::PrimaryWindow,
};

use super::GameState;

use crate::tower::{TowerBundle, TowerStats};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(startup.in_schedule(OnEnter(GameState::Game)))
        //.add_system(show_towers)
           .add_system(place_tower);
    }
}

fn startup(mut commands: Commands,    
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
        ..default()
    });
}

fn show_towers(mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    towers_query: Query<&TowerStats>) {
        for tower in towers_query.iter() {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_xyz(tower.y, tower.x, 0.),
            ..default()
        });
    }
    }

fn place_tower(mut commands: Commands, 
    mut mouse_button_input: Res<Input<MouseButton>>, 
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,) {

    let Ok(window) = primary_window_query.get_single() else {
            return;
    };

    if let Some(_position) = window.cursor_position() {
        if mouse_button_input.just_released(MouseButton::Left) {
            info!("left mouse just released");
            info!("{} {}", _position.x, _position.y);
            let x = _position.x - window.width() / 2.0;
            let y = _position.y - window.height() / 2.0;
            commands.spawn((TowerBundle::new(x, y), MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(50.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_xyz(x, y, 0.),
                ..default()
            }));
        }   
     }
    


}