use tcod::console::*;
use tcod::colors::*;

use crate::rustpunk::pos::Pos;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub solid: bool,
    pub opaque: bool,
    pub char: char,
    pub color: Color,
    pub explored: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            solid: false,
            opaque: false,
            char: '.',
            color: GREY,
            explored: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            solid: true,
            opaque: true,
            char: '#',
            color: WHITE,
            explored: false,
        }
    }

    pub fn draw(&self, pos: Pos, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(pos.x, pos.y, self.char, BackgroundFlag::None);
    }

    pub fn draw_fow(&self, pos: Pos, con: &mut dyn Console) {
        con.set_default_foreground(self.color.scale_hsv(0.2, 0.2));
        con.put_char(pos.x, pos.y, self.char, BackgroundFlag::None);
    }
}
