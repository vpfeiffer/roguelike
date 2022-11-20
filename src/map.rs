use rltk::{RGB, Rltk, RandomNumberGenerator, Algorithm2D, BaseMap, Point};
use specs::{World, WorldExt, Join};
use super::{Rect, Player, Viewshed };
use std::cmp::{max, min};

const WIDTH: usize = 80;
const HEIGHT: usize = 50;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

#[derive(Default)]
pub struct Map {
    pub tiles : Vec<TileType>,
    pub rooms : Vec<Rect>,
    pub width : i32,
    pub height : i32,
}

impl Map {

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * WIDTH) + x as usize
    }
    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
    fn apply_horizontal_tunnel(&mut self, x1:i32, x2:i32, y:i32) {
        for x in min(x1, x2) ..= max(x1,x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
    fn apply_vertical_tunnel(&mut self, y1:i32, y2:i32, x:i32) {
        for y in min(y1, y2) ..= max(y1,y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map {
            tiles : vec![TileType::Wall; WIDTH*HEIGHT],
            rooms : Vec::new(),
            width : WIDTH as i32,
            height : HEIGHT as i32,
        };

        const MAX_ROOMS : i32 = 30;
        const MIN_SIZE : i32 = 6;
        const MAX_SIZE : i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            // Refactor the following two lines to use rng.range and subtract just 1 instead
            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            let mut not_intersect = true;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) { 
                    not_intersect = false;
                }
            }
            if not_intersect {
                map.apply_room_to_map(&new_room);
                
                // Join rooms with corridors
                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len()-1].center();
                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }

                }
                map.rooms.push(new_room);
            }
        }
        map
    }
}


pub fn draw_map(ecs: &World, ctx : &mut Rltk) {
    let map = ecs.fetch::<Map>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let mut players = ecs.write_storage::<Player>();
    
    for (_player, viewshed) in (&mut players, &mut viewsheds).join() {
        let mut y = 0;
        let mut x = 0;
        for tile in map.tiles.iter() {
            // Render a tile depending upon the tile type
            let pt = Point::new(x, y);
            if viewshed.visible_tiles.contains(&pt) {
                match tile {
                    TileType::Floor => {
                        ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
                    }
                }

                // Move the coordinates
                x += 1;
                if x > 79 {
                    x = 0;
                    y += 1;
                }
            }
        }
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx:usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall

    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> rltk::Point {
        Point::new(self.width, self.height)
    }
}

