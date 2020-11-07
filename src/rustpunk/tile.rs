use tcod::console::*;
use tcod::colors::*;

use crate::rustpunk::pos::Pos;

#[derive(Clone, Copy, Debug)]
/// Structure for storing information about tiles
pub struct Tile {
    pub solid: bool,
    pub opaque: bool,
    pub char: char,
    pub color: Color,
    pub explored: bool,
}

impl Tile {
    /// Default empty tile
    pub fn empty() -> Self {
        Tile {
            solid: false,
            opaque: false,
            char: '.',
            color: LIGHTEST_GREY,
            explored: false,
        }
    }

    /// Default wall tile
    pub fn wall() -> Self {
        Tile {
            solid: true,
            opaque: true,
            char: '#',
            color: WHITE,
            explored: false,
        }
    }

    /// Draws the tile
    pub fn draw(&self, pos: Pos, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.set_default_background(LIGHT_GREY);
        con.put_char(pos.x, pos.y, self.char, BackgroundFlag::Set);
    }

    /// Draws the tile with unsaturated colors (e.g. for tiles that are explored
    /// but not visible)
    pub fn draw_fow(&self, pos: Pos, con: &mut dyn Console) {
        con.set_default_foreground(self.color.scale_hsv(0.2, 0.2));
        con.set_default_background(GREY);
        con.put_char(pos.x, pos.y, self.char, BackgroundFlag::Set);
    }

    pub fn draw_unexplored(&self, pos: Pos, con: &mut dyn Console) {
        con.set_default_background(BLACK);
        con.put_char(pos.x, pos.y, ' ', BackgroundFlag::Set);
    }
}
