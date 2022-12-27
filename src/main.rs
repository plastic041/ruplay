use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::time::FixedTimestep;

use rand::prelude::*;

#[derive(Component, Clone)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component, PartialEq, Clone)]
enum State {
    Alive,
    Dead,
}

#[derive(Component, Clone)]
struct Cell {
    position: Position,
    state: State,
}

impl Cell {
    fn new(x: i32, y: i32) -> Self {
        let mut rng = thread_rng();
        let state = if rng.gen_bool(0.5) {
            State::Alive
        } else {
            State::Dead
        };
        Self {
            position: Position { x, y },
            state,
        }
    }
}

const COLS: i32 = 70;
const ROWS: i32 = 70;
const CELL_SIZE: f32 = 4.;

const WORLD_WIDTH: f32 = COLS as f32 * CELL_SIZE;
const WORLD_HEIGHT: f32 = ROWS as f32 * CELL_SIZE;

fn setup(mut commands: Commands) {
    // Add a 2d camera
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::DARK_GREEN),
        },
        ..default()
    });

    // Add cells to the world
    for y in 0..COLS {
        for x in 0..ROWS {
            let cell = Cell::new(x, y);
            let color = match cell.state {
                State::Alive => Color::WHITE,
                State::Dead => Color::BLACK,
            };
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                        color,
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        x as f32 * CELL_SIZE - WORLD_WIDTH / 2. + CELL_SIZE / 2.,
                        y as f32 * CELL_SIZE - WORLD_HEIGHT / 2. + CELL_SIZE / 2.,
                        0.,
                    )),
                    ..default()
                },
                cell,
            ));
        }
    }
}

fn update_cells(mut query: Query<(&mut Cell, &mut Sprite)>) {
    let cells = query.iter().map(|q| q.0).cloned().collect::<Vec<_>>();

    for (mut cell, mut sprite) in query.iter_mut() {
        // Update the cell's sprite color
        sprite.color = match cell.state {
            State::Alive => Color::WHITE,
            State::Dead => Color::BLACK,
        };

        // Update the cell's state
        let neighbors = get_neighbors(&cell.position);

        let mut alive_neighbors = 0;

        for neighbor in neighbors {
            if let Some(neighbor_cell) = get_cell(&neighbor, &cells) {
                if neighbor_cell.state == State::Alive {
                    alive_neighbors += 1;
                }
            }
        }

        if cell.state == State::Alive {
            if !(2..=3).contains(&alive_neighbors) {
                cell.state = State::Dead;
            }
        } else if alive_neighbors == 3 {
            cell.state = State::Alive;
        }
    }
}

fn get_neighbors(position: &Position) -> Vec<Position> {
    let mut neighbors = Vec::new();

    for y in -1..=1 {
        for x in -1..=1 {
            if x == 0 && y == 0 {
                continue;
            }

            let neighbor = Position {
                x: position.x + x,
                y: position.y + y,
            };

            if neighbor.x < 0 || neighbor.x >= COLS || neighbor.y < 0 || neighbor.y >= ROWS {
                continue;
            }

            neighbors.push(neighbor);
        }
    }

    neighbors
}

fn get_cell<'a>(position: &'a Position, cells: &'a [Cell]) -> Option<&'a Cell> {
    let index = position.y * COLS + position.x;
    let cell = cells.get(index as usize).unwrap();

    Some(cell)
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Cells".to_string(),
                width: 500.,
                height: 300.,
                ..default()
            },
            ..default()
        }))
        .add_system(bevy::window::close_on_esc)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.04))
                .with_system(update_cells),
        )
        .add_startup_system(setup)
        .run();
}
