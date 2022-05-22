use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

/* By using  #[derive(Component)] you avoid the need to type the following:
 * impl Component for Position {
 *      type Storage = VecStorage<Self>;
 *  } */


fn main() {
    println!("Hello, world!");
}
