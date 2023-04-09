
use std::cell::Cell;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::sprite::MaterialMesh2dBundle;
use super::GameState;
use bevy::utils::HashMap;
use rand::Rng;


pub const CELL_SIZE: f32 = 30.;  // probably should be an even number for the math to work

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(build_map.in_schedule(OnEnter(GameState::Game)))
        .add_system(update_map);
    }
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct CellCoordinate {
    pub x: i32,
    pub y: i32
}

#[derive(Component)]
pub struct Wall;

#[derive(Component, Default, Debug)]
pub struct Map {
    width: u32,
    height: u32,
    walls: HashMap<CellCoordinate, bool>,
    pub came_from: HashMap<CellCoordinate, CellCoordinate>
}

// Queue from https://www.kirillvasiltsov.com/writing/how-to-write-a-queue-in-rust/
struct Queue<T> {
    queue: Vec<T>,
  }

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    fn length(&self) -> usize {
        self.queue.len()
    }

    fn enqueue(&mut self, item: T) {
        self.queue.push(item)
    }

    fn dequeue(&mut self) -> T {
        self.queue.remove(0)
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn peek(&self) -> Option<&T> {
        self.queue.first()
    }
}

impl Map {
    pub fn has_wall(&self, coordinate: &CellCoordinate) -> bool {
        match self.walls.get(coordinate) {
            Some(&answer) => answer,
            None => false
        }
    }

    pub fn set_wall(&mut self, coordinate: CellCoordinate, new_value: bool){
        if let Some(value) = self.walls.get_mut(&coordinate) {
            *value = new_value;
        } else {
            self.walls.insert(coordinate, new_value);
        }
    }

    pub fn in_map(&self, coordinate: CellCoordinate) -> bool {
        let half_width: i32 = (self.width/2) as i32;
        let half_height: i32 = (self.height/2) as i32;

        (-half_width <= coordinate.x) && (coordinate.x <= half_width) && (-half_height <= coordinate.y) && (coordinate.y <= half_height)
    }

    pub fn get_neighbors(&self, coordinate: CellCoordinate) -> Vec<CellCoordinate> {
        let mut output = vec![];
        // for dx in vec![-1, 0, 1] {
        //     for dy in vec![-1, 0, 1] {
            for (dx, dy) in vec![(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let neighbor = CellCoordinate{x: coordinate.x + dx, y: coordinate.y + dy};
                    if self.in_map(neighbor) && !self.has_wall(&neighbor) {
                        output.push(neighbor);
                    }
                }
        output
    }
}


fn build_map(
    mut commands: Commands,
    primary_window_query: Query<&Window, With<PrimaryWindow>>, 
    mouse_button_input: Res<Input<MouseButton>>, 
    game_state: Res<State<GameState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>) {

    if game_state.0 == GameState::Game {
        let Ok(window) = primary_window_query.get_single() else {
            return;
        };

        let mut map = Map{width: (window.width() / CELL_SIZE) as u32,
                                height: (window.height() / CELL_SIZE) as u32,
                             walls: HashMap::new(), 
                             came_from: HashMap::new()};

        let half_width: i32 = (map.width/2) as i32;
        let half_height: i32 = (map.height/2) as i32;

        for _ in 1..100 {
            let mut rng = rand::thread_rng();

            let x = rng.gen_range(-half_width..half_width);
            let y = rng.gen_range(-half_height..half_height);

            let dx = rng.gen_range(0..4);
            let dy = rng.gen_range(0..4);
            
            for cell_x in x..x+dx {
                map.set_wall(CellCoordinate { x: cell_x, y: y}, true);
            }

            for cell_y in y..y+dy {
                map.set_wall(CellCoordinate {x: x, y: cell_y}, true);
            }
        }

        for x in -2..2 {
            for y in -2..2 {
                map.set_wall(CellCoordinate { x: x, y: y }, false);
            }
        }


        for (coordinate, &has_wall) in map.walls.iter() {
            if has_wall {
                commands.spawn((MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(CELL_SIZE, CELL_SIZE)))).into(),
                    transform: Transform::default().with_translation(Vec3::new(coordinate.x as f32 * CELL_SIZE, 
                                                                                coordinate.y as f32 * CELL_SIZE, 
                                                                                2.)),
                    material: materials.add(ColorMaterial::from(Color::GOLD)),
                    ..default()
                }, Wall));
            }
        }

        let mut frontier = Queue::<CellCoordinate>::new();
        frontier.enqueue(CellCoordinate{x: 0, y:0});
        map.came_from.insert(CellCoordinate{x: 0, y: 0},  CellCoordinate{x: 0, y: 0});

        // for _ in 1..1000 {
        while !frontier.is_empty() {
            info!("map size {} {}", map.width, map.height);
            info!("frontier size: {}", frontier.length());
            let current = frontier.dequeue();
            info!("current {} {}", current.x, current.y);
            for next in map.get_neighbors(current) {
                if !map.came_from.contains_key(&next) {
                    frontier.enqueue(next);
                    map.came_from.insert(next, current);
                }
            }
        }

        // for (key, value) in map.came_from.iter() {
        //     commands.spawn((
        //         MaterialMesh2dBundle {
        //             mesh: meshes.add(shape::Circle::new(3.).into()).into(),
        //             material: materials.add(ColorMaterial::from(Color::GREEN)),
        //             transform: Transform::from_xyz(key.x as f32 * CELL_SIZE, key.y as f32 * CELL_SIZE, 0.),
        //             ..default()
        //         }));        
        //     }

        commands.spawn(map);
    }
}

fn update_map(
    mut commands: Commands,
    primary_window_query: Query<&Window, With<PrimaryWindow>>, 
    mouse_button_input: Res<Input<MouseButton>>, 
    game_state: Res<State<GameState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>) {

    if game_state.0 == GameState::Game {

        let Ok(window) = primary_window_query.get_single() else {
                return;
        };

        if let Some(_position) = window.cursor_position() {
            if mouse_button_input.just_pressed(MouseButton::Right) {
                let x = _position.x - window.width() / 2.0;
                let y = _position.y - window.height() / 2.0;
                info!("right click at {} {}", x, y);
                let cell_x = ((x + (CELL_SIZE / 2.0)) / CELL_SIZE).floor();
                let cell_y = ((y + (CELL_SIZE / 2.0)) / CELL_SIZE).floor();
                info!("right click on cell {} {}", cell_x, cell_y);

                commands.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(CELL_SIZE, CELL_SIZE)))).into(),
                    transform: Transform::default().with_translation(Vec3::new(cell_x*CELL_SIZE, cell_y*CELL_SIZE, 0.)),
                    material: materials.add(ColorMaterial::from(Color::GOLD)),
                    ..default()
                });
        }
    }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_neighbors_of_origin() {
        use super::CELL_SIZE;
        use super::{Map, CellCoordinate};
        use bevy::utils::HashMap;

        let mut map = Map{width: (800. / CELL_SIZE) as u32,
            height: (400. / CELL_SIZE) as u32,
         walls: HashMap::new(), 
         came_from: HashMap::new()
        };

        let neighbors = map.get_neighbors(CellCoordinate { x: 0, y: 0 });
        assert!(neighbors.len() == 4);
    }

    #[test]
    fn check_neighbors_of_out_of_bounds() {
        use super::CELL_SIZE;
        use super::{Map, CellCoordinate};
        use bevy::utils::HashMap;

        let mut map = Map{width: (800. / CELL_SIZE) as u32,
            height: (400. / CELL_SIZE) as u32,
         walls: HashMap::new(), 
         came_from: HashMap::new()
        };

        let neighbors = map.get_neighbors(CellCoordinate { x: 100, y: 100 });
        assert!(neighbors.len() == 0);
    }
}
