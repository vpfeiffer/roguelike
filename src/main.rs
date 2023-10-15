#![allow(unused)]
use colored::Colorize;
use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ndarray::{prelude::*, OwnedRepr};
use rand::{prelude::*, distributions::WeightedIndex};
use std::{
    fmt::{format, write},
    io::{self, stdout, Read, Write},
};

//mod map_entities;
//pub use map_entities::*;

// TODO: somehow merge this data with the Room type
#[derive(Clone, Debug, PartialEq, Eq)]
struct RoomCoordinates {
    pub upper_left: (usize, usize),
    pub upper_right: (usize, usize),
    pub lower_left: (usize, usize),
    pub lower_right: (usize, usize)
}

#[derive(Clone, PartialEq, Copy, Debug)]
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

#[derive(Clone, PartialEq, Copy, Debug)]
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
// maybe change Room type to struct with information
// for creating a room later
type Room = Array2<Tile>;

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

fn get_coordinate_modifiers(direction: Movement) -> (i32, i32) {
    match direction {
        Movement::Down => (1, 0),
        Movement::Up => (-1, 0),
        Movement::Left => (0, -1),
        Movement::Right => (0, 1),
        Movement::DiagonalUpLeft => (-1, -1),
        Movement::DiagonalUpRight => (-1, 1),
        Movement::DiagonalDownLeft => (1, -1),
        Movement::DiagonalDownRight => (1, 1),
    }
}

fn get_coordinates(player_location: (usize, usize), direction: Movement) -> (usize, usize) {
    let coordinate_modifiers = get_coordinate_modifiers(direction);
    (
        (player_location.0 as i32 + coordinate_modifiers.0) as usize,
        (player_location.1 as i32 + coordinate_modifiers.1) as usize,
    )
}

fn move_player(map: &mut Map, direction: Movement) {
    let player_location = find_player(map).unwrap();
    let coordinates = get_coordinates(player_location, direction);

    if map[[coordinates.0, coordinates.1]] == Tile::Floor {
        map[[player_location.0, player_location.1]] = Tile::Floor;
        map[[coordinates.0, coordinates.1]] = Tile::Player;
    }
}

fn determine_player_movement(map: &mut Map, user_input: &Option<KeyCode>) {
    match user_input {
        Some(KeyCode::Char('j')) => move_player(map, Movement::Down),
        Some(KeyCode::Char('k')) => move_player(map, Movement::Up),
        Some(KeyCode::Char('h')) => move_player(map, Movement::Left),
        Some(KeyCode::Char('l')) => move_player(map, Movement::Right),
        Some(KeyCode::Char('y')) => move_player(map, Movement::DiagonalUpLeft),
        Some(KeyCode::Char('u')) => move_player(map, Movement::DiagonalUpRight),
        Some(KeyCode::Char('b')) => move_player(map, Movement::DiagonalDownLeft),
        Some(KeyCode::Char('n')) => move_player(map, Movement::DiagonalDownRight),
        _ => {}
    }
}

fn find_player(map: &Map) -> Option<(usize, usize)> {
    map.indexed_iter()
        .find(|(_, x)| x == &&Tile::Player)
        .map(|y| y.0)

}

fn keyboard_event() -> Option<KeyCode> {
    match read().unwrap() {
        Event::Key(event) => Some(event.code),
        //Event::Resize(width, height) => print!("New size {}x{}", width, height),
        _ => None,
    }
}

fn create_map() -> ArrayBase<OwnedRepr<Tile>, ndarray::Dim<[usize; 2]>> {
    let mut rng = thread_rng();
    let max_number_of_rooms = 8;
    let min_number_of_rooms = 4;
    let mut first_room = true;
    // Create map of fixed size and of wall tiles.
    // later "carve" out rooms in the map by replacing the wall tiles with floor tiles
    let mut map = Array::from_elem((40, 80), Tile::Wall);
    for _ in 0..rng.gen_range(min_number_of_rooms..max_number_of_rooms) {
        let room = generate_room(&mut rng, first_room);
        add_room_to_map(room, &mut map);
        first_room = false;
    }
    add_hallway_to_map(&mut map);
    place_player_on_map(&mut map);
    return map;
}

fn add_room_to_map(room: Room, map: &mut Map) {
    let mut rng = thread_rng();
    let room_dimensions = room.shape();
    let offset1 = rng.gen_range(0..40 - room_dimensions[0]); // 0..height_map - height_room
    let offset2 = rng.gen_range(0..80 - room_dimensions[1]); // 0..witdth_map - width_room
    for tile in room.indexed_iter() {
        map[[tile.0 .0 + offset1, tile.0 .1 + offset2]] = *tile.1;
    }
}

fn find_empty_floor_tile(map: &Map) -> Option<(usize, usize)>{
    map.indexed_iter()
        .find(|(_, x)| x == &&Tile::Floor)
        .map(|y| y.0)
}

fn place_player_on_map(map: &mut Map) {
    let empty_tile = find_empty_floor_tile(map).unwrap();
    if map[[empty_tile.0, empty_tile.1]] == Tile::Floor {
        map[[empty_tile.0, empty_tile.1]] = Tile::Player;
    }
}

fn generate_room(rng: &mut ThreadRng, first_room: bool) -> Array2<Tile> {
    let width = rng.gen_range(6..10);
    let height = rng.gen_range(6..10);
    let mut room = Array::from_shape_fn((height, width), |(i, j)| {
        if i == 0 || j == 0 || j == width - 1 || i == height - 1 {
            Tile::Wall
        }
        else {
            Tile::Floor
        }
    });
    return room;
}

fn add_hallway_to_map(map: &mut Map) {
    // search for two empty_tiles
    // send coordinates to generate_hallway
    for _ in 0..6 {
        let mut entry = find_empty_floor_tile(map).unwrap();
        path_find(map, entry);
    }

}

fn valid_tile(tile: (usize, usize)) -> bool {
    if tile.0 >= 39 || tile.1 >= 79 {
        return false;
    } else {
        return true;
    }

}

fn list_rooms(map: &Map) -> Vec<RoomCoordinates> {
    // return a list of rooms in the map
    // to be used with path_find()
    // traverse map
    // find floor tiles in a line, stop when you find a floor tile that's right before a wall tile
    // for the width
    // use indexed_iter

    let mut rooms_list = Vec::new();
    let map_iter = map.indexed_iter();
    // These are now the option type
    let mut upper_left = None;
    let mut upper_right = None;
    let mut lower_left = None; 
    let mut lower_right = None;
    for (i,tile) in map_iter {
        if tile == &Tile::Wall && i.1 + 1 < map.shape()[1] && upper_left == None && map[(i.0,i.1 +1 as i32 as usize)] == Tile::Floor {
            upper_left = Some((i.0,i.1 + 1));
            for j in 1..(map.shape()[0] - upper_left.unwrap().0) {
                if map[(upper_left.unwrap().0 + j, upper_left.unwrap().1)] == Tile::Floor && upper_left.unwrap().0 + j < map.shape()[0] && lower_left == None && map[(upper_left.unwrap().0 + 1 as i32 as usize, i.1)] == Tile::Wall {
                    lower_left = Some((upper_left.unwrap().0 + j, upper_left.unwrap().1));
                }
            }
        }
        if tile == &Tile::Floor && i.1 + 1 < map.shape()[1] && upper_right == None && map[(i.0,i.1 +1 as i32 as usize)] == Tile::Wall {
            upper_right = Some(i);
            for j in 0..map.shape()[1] {
                if tile == &Tile::Floor && i.0 + j + 1 < map.shape()[0] && map[(i.0 + j + 1 as i32 as usize, i.1)] == Tile::Wall {
                    lower_right = Some((i.0 + j, i.1));
                }
            }
        }

    }
    
    rooms_list.push(RoomCoordinates { upper_left:upper_left.unwrap(), upper_right:upper_right.unwrap(), lower_left:lower_left.unwrap(), lower_right:lower_right.unwrap() });
    if upper_left == None && upper_right == None && lower_left == None && lower_right == None {
        return Vec::new();

    }
    return rooms_list;

}

fn path_find(map: &mut Map, mut path: (usize, usize)) {
    let mut rng = thread_rng();
    let directions = [(Movement::Down, 4), (Movement::Up, 1), (Movement::Left, 2), (Movement::Right, 4), (Movement::DiagonalUpLeft, 1), (Movement::DiagonalUpRight, 1), (Movement::DiagonalDownLeft, 4), (Movement::DiagonalDownRight, 4)];
    let dist = WeightedIndex::new(directions.iter().map(|direction| direction.1)).unwrap();
    // Refactor this later
    for _ in 0..20 {
        let direction = directions[dist.sample(&mut rng)].0;
        let modifiers = get_coordinate_modifiers(direction);
        let new_path = ((path.0 as i32 + modifiers.0) as usize, (path.1 as i32 + modifiers.1) as usize);
        while valid_tile(new_path) == false {
            let direction = directions[dist.sample(&mut rng)].0;
            let modifiers = get_coordinate_modifiers(direction);
            let new_path = ((path.0 as i32 + modifiers.0) as usize, (path.1 as i32 + modifiers.1) as usize);
        }
        path = ((path.0 as i32 + modifiers.0) as usize, (path.1 as i32 + modifiers.1) as usize);
        map[[path.0, path.1]] = Tile::Floor;
    }
}

fn main() {
    // load game save if it exists

    // TODO: Add hallways connecting each room.
    //       Make them narrow and long rooms that
    //       go from the center of one room to another.

    let mut user_input = None;

    let mut map = create_map();
    enable_raw_mode();
    while user_input != Some(KeyCode::Char('q')) {
        user_input = keyboard_event();
        determine_player_movement(&mut map, &user_input);
        print_map(&map);
    }
    disable_raw_mode();
}

#[cfg(test)]
mod tests {
    // TODO: write test for no rooms
    // TODO: write test for multiple rooms
    use super::*;

    #[test]
    fn test_list_rooms() {
        assert_eq!(list_rooms(&arr2(&[[Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
                                        [Tile::Wall, Tile::Floor, Tile::Floor, Tile::Wall],
                                        [Tile::Wall, Tile::Floor, Tile::Floor, Tile::Wall],
                                        [Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall]
                                ])), [RoomCoordinates { upper_left:(1,1), upper_right:(1,2), lower_left:(2,1), lower_right:(2,2) }]);
    }


}

