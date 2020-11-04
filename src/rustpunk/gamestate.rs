use std::cmp::Ordering;
use core::cmp::max;
use array2d::Array2D;

use crate::rustpunk::tile::Tile;
use crate::rustpunk::object::*;
use crate::rustpunk::pos::Pos;

use tcod::console::*;
use tcod::colors::*;
use tcod::bsp::*;
use tcod::random::Rng;
use tcod::map::FovAlgorithm;

const MAP_SIZE: i32 = 128;

/// Map and related data.
pub struct Map {
    map: Array2D<Tile>,
    tcod_map: tcod::Map,
}

impl Map {
    fn new(map: Array2D<Tile>) -> Self {
        let tcod_map = tcod::Map::new(MAP_SIZE, MAP_SIZE);
        let mut m = Map { map, tcod_map };
        for x in 0..MAP_SIZE {
            for y in 0..MAP_SIZE {
                let tile = m.map
                    .get(x as usize, y as usize)
                    .expect("Tile out of bounds");
                m.tcod_map.set(x, y, !tile.opaque, tile.solid);
            }
        }
        m
    }

    fn get_tile(&self, pos: Pos) -> Tile {
        match self.map.get(pos.x as usize, pos.y as usize) {
            Some(x) => *x,
            None    => Tile::wall(),
        }
    }

    fn get_tile_mut(&mut self, pos: Pos) -> Option<&mut Tile> {
        self.map.get_mut(pos.x as usize, pos.y as usize)
    }

    pub fn is_solid(&self, pos: Pos) -> bool {
        self.get_tile(pos).solid
    }
}

/// The game state structure contains everything that would
/// need to be stored in the savefile when the game is saved.
pub struct GameState {
    map: Map,
    objects: Vec<Object>,
}

impl GameState {
    /// Instantiates a fresh game state
    pub fn new() -> Self {
        let map = make_map();
        let mut gs = GameState {
            map: map,
            objects: Vec::new(),
        };
        gs.populate();
        gs
    }

    /// Randomly populates the map with NPCs
    pub fn populate(&mut self) {
        let mut npcs = Vec::new();
        let rng = Rng::get_instance();
        loop {
            let pos = Pos::new(
                rng.get_int(0, MAP_SIZE-1), 
                rng.get_int(0, MAP_SIZE-1));
            if self.is_walkable(pos) {
                let player = Object::new(pos, '@', WHITE);
                npcs.push(player);
                break
            }
        }
        for _ in 1..100 {
            for _ in 1..1000 {
                let pos = Pos::new(
                    rng.get_int(0, MAP_SIZE-1), 
                    rng.get_int(0, MAP_SIZE-1));
                if self.is_walkable(pos) {
                    let mut npc = Object::new(pos, '@', BLUE);
                    npc.faction = Faction::Enemy;
                    npcs.push(npc);
                    break
                }
            }
        }
        self.objects = npcs;
    }

    /// Advances the game state by one tick.
    pub fn update(&mut self) {
        // Update objects
        let ref objs = self.objects;
        for i in 0..objs.len() {
            let o = objs[i];
            match o.action {
                Action::Idle      => {}
                Action::Move(pos) => {
                    let new_pos = pos + o.pos;
                    if self.is_walkable(new_pos) {
                        let ref mut o_mut = self.objects[i];
                        o_mut.pos = new_pos;
                        o_mut.action = Action::Idle;
                        break;
                    }
                    match objs.iter().position(|x| x.pos == new_pos) {
                        Some(idx) => {
                            let (o_mut, other) = mut_two(i, idx, &mut self.objects[..])
                                .expect("Object is trying to move onto itself");
                            other.health -= o.attack;
                            o_mut.action = Action::Idle;
                            println!("Attacking, {} HP left", other.health);
                            break;
                        }
                        None => {}
                    }
                }
            }
        }

        // Update FOV
        let player_pos = self.get_player().pos;
        self.map.tcod_map.compute_fov(
            player_pos.x, 
            player_pos.y, 
            32, 
            true, 
            FovAlgorithm::Basic);

        // Update fog of war
        for x in 0..MAP_SIZE {
            for y in 0..MAP_SIZE {
                if self.map.tcod_map.is_in_fov(x, y) {
                    let pos = Pos::new(x, y);
                    match self.map.get_tile_mut(pos) {
                        Some(x) => x.explored = true,
                        None    => {},
                    }
                }
            }
        }
    }

    /// Checks whether the tile at position `pos` is currently visible to
    /// the player.
    fn is_visible(&self, pos: Pos) -> bool {
        let in_bounds = 
            pos.x >=0 &&
            pos.y >=0 &&
            pos.x < MAP_SIZE &&
            pos.y < MAP_SIZE;
            if in_bounds {
                self.map.tcod_map.is_in_fov(pos.x, pos.y)
            } else {
                false
            }
    }

    /// Renders all tiles and game objects on the screen.
    pub fn render(&self, cam_pos: Pos, con: &mut dyn Console) {
        con.set_default_foreground(WHITE);
        con.clear();
        for sx in 0..con.width() {
            for sy in 0..con.height() {
                let pos = Pos::new(sx, sy);
                let wpos = pos + cam_pos;
                let tile = self.map.get_tile(wpos);
                if self.is_visible(wpos) {
                    tile.draw(pos, con);
                } else if tile.explored {
                    tile.draw_fow(pos, con);
                }
            }
        }
        for o in &self.objects {
            if self.is_visible(o.pos){
                o.draw(o.pos - cam_pos, con);
            }
        }
    }

    /// Get a mutable reference to the player object.
    pub fn get_player_mut(&mut self) -> &mut Object {
        let player = self.objects.get_mut(0);
        match player {
            Some(p) => p,
            None    => panic!("Player not found!"),
        }
    }

    /// Get a reference to the player object.
    pub fn get_player(&self) -> &Object {
        let player = self.objects.get(0);
        match player {
            Some(p) => p,
            None    => panic!("Player not found!"),
        }
    }

    /// Tell the player object to do a move action on the next tick.
    pub fn player_move(&mut self, delta: Pos) {
        self.get_player_mut().action = Action::Move(delta);
        self.update();
    }

    pub fn is_walkable(&self, pos: Pos) -> bool {
        !self.map.is_solid(pos) && !self.object_at(pos).is_some()
    }


    pub fn object_at(&self, pos: Pos) -> Option<usize> {
        self.objects.iter().position(|x| x.pos == pos)
    }
}

/// Generates a map using binary space partitioning. This will result in
/// a map that resembles city streets.
fn make_map() -> Map {
    const STREET_WIDTH: i32 = 4;
    let mut map = Array2D::filled_with(
        Tile::wall(), 
        MAP_SIZE as usize, 
        MAP_SIZE as usize);
    let rng = &mut Rng::get_instance();
    let bsp = &mut Bsp::new_with_size(0, 0, MAP_SIZE, MAP_SIZE);
    bsp.split_recursive(Some(rng), 5, 20, 20, 1., 1.);
    bsp.traverse(TraverseOrder::InOrder, |b| {
        for x in b.x..b.x+b.w {
            for y in b.y..b.y+b.h {
                let draw_wall =
                    x - b.x >= STREET_WIDTH && 
                    y - b.y >= STREET_WIDTH;
                if !draw_wall {
                    map.set(x as usize, y as usize, Tile::empty())
                        .expect("Could not set tile");
                }
            }
        }
        true
    });
    Map::new(map)
}

/// Mutably borrow two *separate* elements from the given slice.
/// Panics when the indexes are equal or out of bounds.
fn mut_two<T>(
    first_index: usize, 
    second_index: usize, 
    items: &mut [T]) -> Option<(&mut T, &mut T)> {

    let split_at_index = max(first_index, second_index);
    let (first_slice, second_slice) = items.split_at_mut(split_at_index);
    match first_index.cmp(&second_index) {
        Ordering::Less    => Some((&mut first_slice[first_index], &mut second_slice[0])),
        Ordering::Greater => Some((&mut second_slice[0], &mut first_slice[second_index])),
        Ordering::Equal   => None,
    }
}
