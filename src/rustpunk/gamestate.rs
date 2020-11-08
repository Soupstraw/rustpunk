use core::cell::*;
use std::cmp::Ordering;
use core::cmp::max;
use array2d::Array2D;

use crate::rustpunk::tile::Tile;
use crate::rustpunk::object::*;
use crate::rustpunk::pos::*;
use crate::rustpunk::message::Message;

use tcod::line::*;
use tcod::console::*;
use tcod::colors::*;
use tcod::random::Rng;
use tcod::map::FovAlgorithm;

const MAP_SIZE: i32 = 128;
const VIEWPORT_WIDTH: i32 = 80;
const VIEWPORT_HEIGHT: i32 = 50;
const MSG_DISPLAY_COUNT: i32 = 5;
const HEALTH_BAR_WIDTH: i32 = 20;
const HEALTH_BAR_BG_COLOR: Color = DARKEST_RED;
const HEALTH_BAR_FG_COLOR: Color = DARK_RED;
const DEFAULT_BACKGROUND_COLOR: Color = GREY;

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
    objects: Vec<RefCell<Character>>,
    messages: Vec<Message>,
}

impl GameState {
    /// Instantiates a fresh game state
    pub fn new() -> Self {
        let map = make_map();
        let mut gs = GameState {
            map: map,
            objects: Vec::new(),
            messages: Vec::new(),
        };
        gs.populate();
        gs
    }

    /// Randomly populates the map with NPCs
    pub fn populate(&mut self) {
        let rng = Rng::get_instance();
        loop {
            let pos = Pos::new(
                rng.get_int(0, MAP_SIZE-1), 
                rng.get_int(0, MAP_SIZE-1));
            if self.is_walkable(pos) {
                self.objects.push(RefCell::new(Character::player(pos)));
                break
            }
        }
        for _ in 1..20 {
            for _ in 1..1000 {
                let pos = Pos::new(
                    rng.get_int(0, MAP_SIZE-1), 
                    rng.get_int(0, MAP_SIZE-1));
                if self.is_walkable(pos) {
                    self.objects.push(RefCell::new(Character::wolf(pos)));
                    break
                }
            }
        }
    }

    /// Advances the game state by one tick.
    pub fn update(&mut self) {
        // Update objects
        for i in 0..self.objects.len() {
            let ref mut o = self.objects[i].borrow_mut();
            // Do nothing if object is dead
            if !o.alive {
                continue;
            }
            o.update(self);
            let action = o.next_action(self);
            let o_pos = o.pos;
            // Ask the controller for the next action
            match action {
                Action::Idle      => {}
                Action::Move(pos) => {
                    let new_pos = pos.to_pos() + o_pos;
                    if self.is_walkable(new_pos) {
                        // Walk if there is nothing in the way
                        o.pos = new_pos;
                    } else {
                        // Check whether the thing in the way was another object.
                        // If yes, then attack.
                        for j in 0..self.objects.len() {
                            if i == j {
                                // We don't want to attack ourselves
                                continue;
                            }
                            let ref mut other = self.objects[j].borrow_mut();
                            if other.pos == new_pos {
                                let msg = o.attack(other);
                                // Append an attack message
                                self.messages.push(msg);
                                if !other.alive {
                                    let msg = Message::new(format!("{} dies!", other.name));
                                    self.messages.push(msg);
                                }
                            }
                        }
                    }
                }
                Action::Get(item_i) => {
                    for j in 0..self.objects.len() {
                        if i == j {
                            // No point getting stuff from our own inventory
                            continue;
                        }
                        let ref mut other = self.objects[j].borrow_mut();
                        if !other.alive {
                            let item = other.inventory.remove_item(item_i);
                            o.inventory.add_item(item);
                        } else {
                            panic!("Attempting to take stuff from a living being!");
                        }
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
            FovAlgorithm::Diamond);

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

    fn cam_pos(&self) -> Pos {
        self.get_player().pos - Pos::new(VIEWPORT_WIDTH/2, VIEWPORT_HEIGHT/2)
    }

    fn render_object(&self, con: &mut dyn Console, o: &Character) {
        if self.is_visible(o.pos){
            let view_pos = o.pos - self.cam_pos();
            if view_pos.x >= 0 && view_pos.x < VIEWPORT_WIDTH &&
               view_pos.y >= 0 && view_pos.y < VIEWPORT_HEIGHT {
                o.draw(view_pos, con);
            }
        }
    }

    /// Renders the whole screen
    pub fn render(&self, con: &mut Offscreen) {
        self.render_viewport(con);
        self.render_gui(con);
    }

    /// Renders all tiles and game objects on the screen.
    fn render_viewport(&self, con: &mut dyn Console) {
        con.set_default_foreground(WHITE);
        con.clear();
        for sx in 0..VIEWPORT_WIDTH {
            for sy in 0..VIEWPORT_HEIGHT {
                let pos = Pos::new(sx, sy);
                let wpos = pos + self.cam_pos();
                let tile = self.map.get_tile(wpos);
                if self.is_visible(wpos) {
                    tile.draw(pos, con);
                } else if tile.explored {
                    tile.draw_fow(pos, con);
                } else {
                    tile.draw_unexplored(pos, con);
                }
            }
        }
        // Draw non-blocking objects
        for i in 0..self.objects.len() {
            let ref o = self.get_object(i);
            if !o.blocking {
                &self.render_object(con, o);
            }
        }
        // Draw blocking objects
        for i in 0..self.objects.len() {
            let ref o = self.get_object(i);
            if o.blocking {
                &self.render_object(con, o);
            }
        }
    }

    fn render_gui(&self, con: &mut Offscreen) {
        let idx = max(self.messages.len() as i32 - MSG_DISPLAY_COUNT, 0);
        let tail = &self.messages[idx as usize..];
        for i in 0..tail.len() {
            con.set_default_foreground(tail[i].color);
            con.print(
                0, 
                VIEWPORT_HEIGHT - MSG_DISPLAY_COUNT + i as i32, 
                &tail[i].text);
        }
        let player = self.get_player();
        let player_health_ratio: f32 = player.health as f32 / player.max_health() as f32;
        con.set_default_foreground(WHITE);
        con.print(0, VIEWPORT_HEIGHT - MSG_DISPLAY_COUNT - 2, "HP");
        for i in 0..HEALTH_BAR_WIDTH {
            let color = if player_health_ratio <= i as f32 / HEALTH_BAR_WIDTH as f32 {
                HEALTH_BAR_BG_COLOR
            } else {
                HEALTH_BAR_FG_COLOR
            };
            con.set_default_background(color);
            con.put_char(
                i+3,
                VIEWPORT_HEIGHT - MSG_DISPLAY_COUNT - 2,
                ' ',
                BackgroundFlag::Set)
        }
        con.set_default_background(DEFAULT_BACKGROUND_COLOR);
    }

    /// Get a mutable reference to the player object.
    pub fn get_player_mut(&mut self) -> RefMut<Character> {
        self.get_object_mut(0)
    }

    /// Get a reference to the player object.
    pub fn get_player(&self) -> Ref<Character> {
        self.get_object(0)
    }

    pub fn get_object(&self, i: usize) -> Ref<Character> {
        self.objects[i].borrow()
    }

    pub fn get_object_mut(&mut self, i: usize) -> RefMut<Character> {
        self.objects[i].borrow_mut()
    }

    pub fn player_action(&mut self, a: Action) {
        match *self.get_player_mut().controller {
            Controller::PlayerController{ref mut action} => *action = a,
            _ => panic!("Player object does not have a PlayerController"),
        }
        self.update();
    }

    pub fn is_walkable(&self, pos: Pos) -> bool {
        let blocking_object = self
            .object_at_unsafe(pos)
            .map(|i| self.get_object(i).blocking)
            .unwrap_or(false);
        !self.map.is_solid(pos) && !blocking_object
    }

    pub fn object_at_unsafe(&self, pos: Pos) -> Option<usize> {
        for i in 0..self.objects.len() {
            match self.objects[i].try_borrow() {
                Ok(r) => {
                    if r.pos == pos {
                        return Some(i);
                    }
                }
                Err(_) => {}
            }
        }
        None
    }

    pub fn check_los(&self, a: Pos, b: Pos) -> bool {
        let mut line = Line::new(a.tup(), b.tup());
        line.all(|(x, y)| self.is_visible(Pos::new(x, y)))
    }
}

/// Generates a map using binary space partitioning. This will result in
/// a map that resembles city streets.
fn make_map() -> Map {
    let mut map = Array2D::filled_with(
        Tile::empty(), 
        MAP_SIZE as usize, 
        MAP_SIZE as usize);
    let rng = Rng::get_instance();
    for _ in 0..MAP_SIZE*MAP_SIZE/5 {
        let pos = Pos::new(
            rng.get_int(0, MAP_SIZE-1), 
            rng.get_int(0, MAP_SIZE-1));
        map.set(pos.x as usize, pos.y as usize, Tile::wall());
    }
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
