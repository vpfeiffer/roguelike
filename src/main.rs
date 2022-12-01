#![allow(unused)]
use std::io::{self, Read};

mod game_entities;
pub use game_entities::*;

fn get_user_input() -> io::Result<(String)> {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;

    Ok(user_input.trim().to_owned())

}

fn main() {
    // Note: use crossterm or termion for synchronous key press event handling
    //if user_input to do: compare string, compare the input with what the string is for ESC

    // player struct

    // load game save if it exists

    let mut user_input = String::new();

    while user_input != "q" {
        user_input = get_user_input().unwrap();
        println!("game is running...");
        println!("You typed: {}", user_input);
    }

}
