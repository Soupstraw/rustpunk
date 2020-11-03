use crate::rustpunk::pos::Pos;

use tcod::colors::Color;
use tcod::console::*;

#[derive(Clone)]
pub enum Action {
    Idle,
    Move(Pos),
}

#[derive(Clone)]
pub struct Object {
    pub pos: Pos,
    pub char: char,
    pub color: Color,
    pub action: Action,
}

impl Object {
    pub fn new(pos: Pos, char: char, color: Color) -> Self {
        Object { 
            pos: pos, 
            char: char, 
            color: color,
            action: Action::Idle,
        }
    }

    pub fn draw(&self, pos: Pos, con: &mut dyn Console) {
        let in_bounds = 
            pos.x >= 0 && 
            pos.x < con.width() && 
            pos.y >= 0 &&
            pos.y < con.height();
        if in_bounds {
            con.set_default_foreground(self.color);
            con.put_char(pos.x, pos.y, self.char, BackgroundFlag::None);
        }
    }

    pub fn move_by(&mut self, delta: Pos) {
        self.action = Action::Move(delta);
    }
}

