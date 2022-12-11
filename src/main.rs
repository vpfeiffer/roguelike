#![allow(unused)]
use std::io::{self, Read};
use ndarray::prelude::*;

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

fn get_user_input() -> io::Result<(String)> {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;

    Ok(user_input.trim().to_owned())

}

fn print_map(map: &Array2<char>) {

    for row in map.rows()  {
        for tile in row {
            print!("{tile}");
        }
        print!("\n");
    }
}

fn move_player(map: &mut Array2<char>, direction:  Movement, player: &mut GameEntity) {
    map[[player.x, player.y]] = '.';
    match direction {
        Movement::Down => {
            map[[player.x+1,player.y]] = '@';
            player.x += 1;
        },
        Movement::Up => {
            map[[player.x-1,player.y]] = '@';
            player.x -= 1;
        },
        Movement::Left => {
            map[[player.x,player.y-1]] = '@';
            player.y -= 1;
        },
        Movement::Right => {
            map[[player.x,player.y+1]] = '@';
            player.y += 1;
        },
        Movement::DiagonalUpLeft => {
            map[[player.x-1,player.y-1]] = '@';
            player.y -= 1;
            player.x -= 1;
        },
        Movement::DiagonalUpRight => {
            map[[player.x-1,player.y+1]] = '@';
            player.y += 1;
            player.x -= 1;
        },
        Movement::DiagonalDownLeft => {
            map[[player.x+1,player.y-1]] = '@';
            player.y -= 1;
            player.x += 1;
        },
        Movement::DiagonalDownRight => {
            map[[player.x+1,player.y+1]] = '@';
            player.y += 1;
            player.x += 1;
        },
    }
}

fn determine_player_movement(map: &mut Array2<char>, user_input: &str, player: &mut GameEntity) {
    match user_input {
        "j" => move_player(map, Movement::Down, player),
        "k" => move_player(map, Movement::Up, player),
        "h" => move_player(map, Movement::Left, player),
        "l" => move_player(map, Movement::Down, player),
        "y" => move_player(map, Movement::DiagonalUpLeft, player),
        "u" => move_player(map, Movement::DiagonalUpRight, player),
        "b" => move_player(map, Movement::DiagonalDownLeft, player),
        "n" => move_player(map, Movement::DiagonalDownRight, player),
        _ => {},
    }
}

fn main() {
    // Note: use crossterm or termion for synchronous key press event handling
    //if user_input to do: compare string, compare the input with what the string is for ESC


    // player struct
    let mut player = GameEntity {
        x: 3,
        y: 3,
    };

    // load game save if it exists

    let mut user_input = String::new();

    // make this an array of chars
    //let mut map = Array2::<char>::default((6,8));

    //map.fill('.');
    let mut map = arr2(&[['#','#','#','#','#','#'],
                           ['#','.','.','.','.','#'],
                           ['#','.','.','.','.','#'],
                           ['#','.','.','@','.','#'],
                           ['#','.','.','.','.','#'],
                           ['#','#','#','#','#','#']]);

    while user_input != "q" {
        user_input = get_user_input().unwrap();
        println!("game is running...");
        println!("You typed: {}", user_input);
        determine_player_movement(&mut map, &user_input, &mut player);
        print_map(&map);
    }

}
