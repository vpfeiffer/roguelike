#![allow(unused)]
use colored::Colorize;
use ndarray::prelude::*;
use std::{io::{self, Read, Write, stdout}, fmt::{format, write}};
use crossterm::{event::{read, Event, KeyCode}, terminal::{enable_raw_mode, disable_raw_mode}};
use rand::{thread_rng, Rng};

mod game_entities;
pub use game_entities::*;
mod map_entities;
pub use map_entities::*;

enum Movement {
    Up,
    Down,
    Left,
    Right,
    DiagonalUpLeft,
    DiagonalUpRight,
    DiagonalDownLeft,
    DiagonalDownRight,
}

#[derive(Clone, PartialEq)]
enum Tile {
    Floor,
    Wall,
    Player,
    NPC,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Floor
    }
}

type Map = Array2<Tile>;

fn print_map(map: &Map) {
    let mut w = io::BufWriter::new(io::stdout());
    write!(w, "\r");
    for row in map.rows() {
        for tile in row {
            match tile {
                Tile::Wall => write!(w, "{}", format!("#").green()),
                Tile::Floor => write!(w, "{}", format!(".").white().dimmed()),
                Tile::Player => write!(w, "{}", format!("@").blue()),
                Tile::NPC => write!(w, "{}", format!("&").red()),
            };
        }
        write!(w, "\n\r");
    }
    write!(w, "\n\r");
}

 fn get_coordinate_modifiers (direction: Movement) -> (i32, i32) {
    match direction {
        Movement::Down => (1,0),
        Movement::Up => (-1,0),
        Movement::Left => (0,-1),
        Movement::Right => (0, 1),
        Movement::DiagonalUpLeft => (-1, -1),
        Movement::DiagonalUpRight => (-1, 1),
        Movement::DiagonalDownLeft => (1, -1),
        Movement::DiagonalDownRight => (1, 1),
    }
 }

fn get_coordinates(player_location: (usize, usize),  direction: Movement) -> (usize, usize) {
    let coordinate_modifiers = get_coordinate_modifiers(direction);
        ((player_location.0 as i32 + coordinate_modifiers.0) as usize , (player_location.1 as i32 + coordinate_modifiers.1) as usize)
}

fn move_player(map: &mut Map, direction: Movement, player: &mut GameEntity) {
    let player_location = find_player(map).unwrap();
    let coordinates = get_coordinates(player_location, direction);

    if map[[coordinates.0, coordinates.1]] == Tile::Floor {
        map[[player_location.0, player_location.1]] = Tile::Floor;
        map[[coordinates.0, coordinates.1]] = Tile::Player;
    }

}

fn determine_player_movement(map: &mut Map, user_input: &Option<KeyCode>, player: &mut GameEntity) {
    match user_input {
        Some(KeyCode::Char('j')) => move_player(map, Movement::Down, player),
        Some(KeyCode::Char('k')) => move_player(map, Movement::Up, player),
        Some(KeyCode::Char('h')) => move_player(map, Movement::Left, player),
        Some(KeyCode::Char('l')) => move_player(map, Movement::Right, player),
        Some(KeyCode::Char('y')) => move_player(map, Movement::DiagonalUpLeft, player),
        Some(KeyCode::Char('u')) => move_player(map, Movement::DiagonalUpRight, player),
        Some(KeyCode::Char('b')) => move_player(map, Movement::DiagonalDownLeft, player),
        Some(KeyCode::Char('n'))=> move_player(map, Movement::DiagonalDownRight, player),
        _ => {}
    }
}

fn find_player(map: &Map) -> Option<(usize, usize)> {
    map.indexed_iter().find(|(_,x)| x == &&Tile::Player).map(|y| y.0)
}

fn keyboard_event() -> Option<KeyCode> {
    match read().unwrap() {
        Event::Key(event) => Some(event.code),
        //Event::Resize(width, height) => print!("New size {}x{}", width, height),
        _ => None
    }
}

fn main() {
    let mut player = GameEntity { x: 3, y: 3 };

    // load game save if it exists
    // TODO: Check if player movement is valid,
    // do not allow movement through walls

    let mut user_input = None;
    let mut rng = thread_rng();
    let width = rng.gen_range(6..14);
    let height = rng.gen_range(6..10);

    let mut map = Array::from_shape_fn((height, width), |(i, j)| {
            if i == 0 || j == 0 || j == width-1 || i == height-1 {
                Tile::Wall
            }
            else if j == width-2 && i == height-2 {
                Tile::Player
            }
            else {
                Tile::Floor
            }
        });

    enable_raw_mode();
    while user_input != Some(KeyCode::Char('q')) {
        user_input = keyboard_event();
        determine_player_movement(&mut map, &user_input, &mut player);
        print_map(&map);
    }
    disable_raw_mode();
}
