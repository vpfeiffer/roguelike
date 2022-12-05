#![allow(unused)]
use std::io::{self, Read};
use ndarray::prelude::*;

mod game_entities;
pub use game_entities::*;

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

fn main() {
    // Note: use crossterm or termion for synchronous key press event handling
    //if user_input to do: compare string, compare the input with what the string is for ESC


    // player struct

    // load game save if it exists

    let mut user_input = String::new();

    // make this an array of chars
    let mut map = Array2::<char>::default((6,8));

    map.fill('#');
    //let mut map = arr2(&[['#','#','#','#','#','#'],
    //                       ['#','.','.','.','.','#'],
    //                       ['#','.','.','.','.','#'],
    //                       ['#','.','.','@','.','#'],
    //                       ['#','.','.','.','.','#'],
    //                       ['#','#','#','#','#','#']]);

    while user_input != "q" {
        user_input = get_user_input().unwrap();
        //dbg!(&map);
        print_map(&map);
        println!("game is running...");
        println!("You typed: {}", user_input);

        //println!("###########");
        //println!("#.........#");
        //println!("#....@....#");
        //println!("#.........#");
        //println!("###########");
    }

}
