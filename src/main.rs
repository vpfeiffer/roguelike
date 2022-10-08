#![allow(unused)]
use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

struct State {
    ecs: World,
}

/* By using  #[derive(Component)] you avoid the need to type the following in addition to the
 * struct
 * impl Component for Position {
 *      type Storage = VecStorage<Self>;
 *  } */

fn main() {

    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();

    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: 40, y: 25 })
            .with(Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
            })
            .build();
    }
}
