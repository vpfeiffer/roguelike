#![allow(unused)]
use colored::Colorize;
use ndarray::prelude::*;
use std::{io::{self, Read, Write, stdout}, fmt::{format, write}};
use crossterm::{event::{read, Event, KeyCode}, terminal::enable_raw_mode};

mod game_entities;
pub use game_entities::*;

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

#[derive(Clone)]
enum Tile {
    Floor,
    Wall,
    Player,
    NPC,
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
}

fn move_player(map: &mut Map, direction: Movement, player: &mut GameEntity) {
    map[[player.x, player.y]] = Tile::Floor;
    match direction {
        Movement::Down => {
            map[[player.x + 1, player.y]] = Tile::Player;
            player.x += 1;
        }
        Movement::Up => {
            map[[player.x - 1, player.y]] = Tile::Player;
            player.x -= 1;
        }
        Movement::Left => {
            map[[player.x, player.y - 1]] = Tile::Player;
            player.y -= 1;
        }
        Movement::Right => {
            map[[player.x, player.y + 1]] = Tile::Player;
            player.y += 1;
        }
        Movement::DiagonalUpLeft => {
            map[[player.x - 1, player.y - 1]] = Tile::Player;
            player.y -= 1;
            player.x -= 1;
        }
        Movement::DiagonalUpRight => {
            map[[player.x - 1, player.y + 1]] = Tile::Player;
            player.y += 1;
            player.x -= 1;
        }
        Movement::DiagonalDownLeft => {
            map[[player.x + 1, player.y - 1]] = Tile::Player;
            player.y -= 1;
            player.x += 1;
        }
        Movement::DiagonalDownRight => {
            map[[player.x + 1, player.y + 1]] = Tile::Player;
            player.y += 1;
            player.x += 1;
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

//fn create_room() {
// fill in this function

//}

fn keyboard_event() -> Option<KeyCode> {
//fn keyboard_event() -> crossterm::event::KeyCode {
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
    //let mut user_input = String::new();

    // make this an array of chars
    //let mut map = Array2::<char>::default((6,8));
    //

    //map.fill('.');
    let mut map = arr2(&[
        [Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
        [Tile::Wall, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall],
        [Tile::Wall, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall],
        [Tile::Wall, Tile::Floor, Tile::Floor, Tile::Player, Tile::Floor, Tile::Wall],
        [Tile::Wall, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall],
        [Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
    ]);

    enable_raw_mode();
    while user_input != Some(KeyCode::Char('q')) {
        user_input = keyboard_event();
        determine_player_movement(&mut map, &user_input, &mut player);
        print_map(&map);
    }
}
