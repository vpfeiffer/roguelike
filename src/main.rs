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

fn move_player(map: &mut Map, direction: Movement, player: &mut GameEntity) {
    let player_location = find_player(map).unwrap();

    map[[player_location.0, player_location.1]] = Tile::Floor;
    match direction {
        Movement::Down => {
            map[[player_location.0 + 1, player_location.1]] = Tile::Player;
        }
        Movement::Up => {
            map[[player_location.0 - 1, player_location.1]] = Tile::Player;
        }
        Movement::Left => {
            map[[player_location.0, player_location.1 - 1]] = Tile::Player;
        }
        Movement::Right => {
            map[[player_location.0, player_location.1 + 1]] = Tile::Player;
        }
        Movement::DiagonalUpLeft => {
            map[[player_location.0 - 1, player_location.1 - 1]] = Tile::Player;
        }
        Movement::DiagonalUpRight => {
            map[[player_location.0 - 1, player_location.1 + 1]] = Tile::Player;
        }
        Movement::DiagonalDownLeft => {
            map[[player_location.0 + 1, player_location.1 - 1]] = Tile::Player;
        }
        Movement::DiagonalDownRight => {
            map[[player_location.0 + 1, player_location.1 + 1]] = Tile::Player;
        }
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
