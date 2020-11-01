use std::ops::Add;

use tcod::colors::*;
use tcod::console::*;

use tcod::input::Key;
use tcod::input::KeyCode as KC;

use array2d::Array2D;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const MAP_SIZE: usize = 128;
const LIMIT_FPS: i32 = 50;

struct Tcod {
    root: Root,
    con: Offscreen,
}

struct Object {
    pos: Pos,
    char: char,
    color: Color,
    movedir: Pos,
}

#[derive(Clone, Copy, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn tup(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object { 
            pos: pos(x, y), 
            char: char, 
            color: color,
            movedir: pos(0, 0),
        }
    }

    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        let (x, y) = self.pos.tup();
        con.put_char(x, y, self.char, BackgroundFlag::None);
    }

    pub fn move_by(&mut self, pos: Pos) {
        self.movedir = pos;
    }
}

#[derive(Clone, Copy, Debug)]
struct Tile {
    solid: bool,
    opaque: bool,
    char: char,
    color: Color,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            solid: false,
            opaque: false,
            char: '.',
            color: GREY,
        }
    }

    pub fn wall() -> Self {
        Tile {
            solid: true,
            opaque: true,
            char: '#',
            color: WHITE,
        }
    }

    pub fn draw(&self, x: i32, y: i32, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(x, y, self.char, BackgroundFlag::None);
    }
}

struct GameState {
    map: Array2D<Tile>,
    objects: Vec<Object>,
}

impl GameState {
    pub fn new() -> Self {
        let player = Object::new(1, 1, '@', WHITE);
        let npc = Object::new(10, 20, '@', BLUE);
        GameState {
            map: make_map(),
            objects: vec![player, npc],
        }
    }

    pub fn update(&mut self) {
        for o in &mut self.objects {
            let pos = o.pos + o.movedir;
            let tile = get_tile(&self.map, pos);
            if !tile.solid {
                o.pos = pos;
            }
        }
    }

    pub fn render(&self, tcod: &mut Tcod) {
        tcod.con.set_default_foreground(WHITE);
        tcod.con.clear();
        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                let tile = get_tile(&self.map, pos(x, y));
                tile.draw(x, y, &mut tcod.con);
            }
        }
        for o in &self.objects {
            o.draw(&mut tcod.con);
        }
        blit(
            &tcod.con, 
            (0, 0), 
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut tcod.root,
            (0, 0),
            1.0,
            1.0
        );
        tcod.root.flush();
    }

    pub fn get_player(&mut self) -> &mut Object {
        let player = self.objects.get_mut(0);
        match player {
            Some(p) => p,
            None    => panic!("Player not found!"),
        }
    }

    pub fn player_move(&mut self, delta: Pos) {
        let p = self.get_player();
        p.move_by(delta);
    }
}

fn main() {
    let root = Root::initializer()
        .font("dejavu16x16_gs_tc.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("rustrogue")
        .init();
    let mut state = GameState::new();

    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut tcod = Tcod { root, con };
    tcod::system::set_fps(LIMIT_FPS);
    
    while !tcod.root.window_closed() {
        state.render(&mut tcod);
        let exit = handle_keys(&mut tcod, &mut state);
        if exit {
            break;
        }
    }
}

fn handle_keys(tcod: &mut Tcod, state: &mut GameState) -> bool {
    // Get the last keypress
    let key = tcod.root.wait_for_keypress(true);

    // Check whether to exit the game
    if key.code == KC::Escape {
        return true
    }

    // Handle movement
    let (dx, dy) = match key {
        Key { printable: 'h', .. } => (-1,  0),
        Key { printable: 'l', .. } => ( 1,  0),
        Key { printable: 'k', .. } => ( 0, -1),
        Key { printable: 'j', .. } => ( 0,  1),
        Key { printable: 'y', .. } => (-1, -1),
        Key { printable: 'u', .. } => ( 1, -1),
        Key { printable: 'b', .. } => (-1,  1),
        Key { printable: 'n', .. } => ( 1,  1),
        _                          => ( 0,  0),
    };

    state.player_move(pos(dx, dy));
    state.update();

    false
}

fn make_map() -> Array2D<Tile> {
    let mut map = Array2D::filled_with(Tile::empty(), MAP_SIZE, MAP_SIZE);
    for i in 0..MAP_SIZE {
        map.set(0, i, Tile::wall());
        map.set(MAP_SIZE-1, i, Tile::wall());
        map.set(i, 0, Tile::wall());
        map.set(i, MAP_SIZE-1, Tile::wall());
    }
    map
}

fn get_tile(map: &Array2D<Tile>, pos: Pos) -> Tile {
    match map.get(pos.x as usize, pos.y as usize) {
        Some(x) => *x,
        None    => Tile::wall()
    }
}

fn pos(x: i32, y: i32) -> Pos {
    Pos {x, y}
}
