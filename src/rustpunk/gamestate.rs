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
        for x in 0..MAP_SIZE-1 {
            for y in 0..MAP_SIZE-1 {
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
        let mut freespot = 0;
        for i in 0..MAP_SIZE-1{
            if !map.get_tile(Pos::new(i, i)).solid {
                freespot = i;
            }
        }
        let player = Object::new(Pos::new(freespot, freespot), '@', WHITE);
        let npc = Object::new(Pos::new(3, 2), '@', BLUE);
        GameState {
            map: map,
            objects: vec![player, npc],
        }
    }

    /// Advances the game state by one tick.
    pub fn update(&mut self) {
        let map = &self.map;

        // Update objects
        for o in &mut self.objects {
            match o.action {
                Action::Idle      => {}
                Action::Move(pos) => {
                    let new_pos = pos + o.pos;
                    let map_collision = map.get_tile(new_pos).solid;
                    if !map_collision {
                        o.pos = new_pos;
                    }
                    o.action = Action::Idle
                }
            }
        }

        // Update FOV
        let player_pos = &self.get_player().pos;
        self.map.tcod_map.compute_fov(
            player_pos.x, 
            player_pos.y, 
            32, 
            true, 
            FovAlgorithm::Basic);

        // Update fog of war
        for x in 0..MAP_SIZE-1 {
            for y in 0..MAP_SIZE-1 {
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
        let p = self.get_player_mut();
        p.move_by(delta);
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
                    map.set(x as usize, y as usize, Tile::empty());
                }
            }
        }
        true
    });
    Map::new(map)
}
