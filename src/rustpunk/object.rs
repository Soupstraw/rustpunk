use crate::rustpunk::pos::Pos;
use crate::rustpunk::message::Message;

use tcod::colors::*;
use tcod::console::*;

#[derive(Clone, Copy)]
pub enum Action {
    Idle,
    Move(Pos),
}

#[derive(Clone, Copy, PartialEq)]
pub enum Faction {
    Neutral,
    Enemy,
}

#[derive(Clone, Copy)]
pub struct Object<'a> {
    pub pos: Pos,
    pub char: char,
    pub color: Color,
    pub name: &'a str,
    pub action: Action,
    pub health: i32,
    pub attack: i32,
    pub faction: Faction,
    pub alive: bool,
    pub blocking: bool,
}

impl<'a> Object<'a> {
    pub fn new(pos: Pos, char: char, color: Color, name: &'a str) -> Object<'a> {
        Object { 
            pos: pos, 
            char: char, 
            color: color,
            name: name,
            action: Action::Idle,
            health: 10,
            attack: 1,
            faction: Faction::Neutral,
            alive: true,
            blocking: true,
        }
    }

    pub fn draw(&self, pos: Pos, con: &mut dyn Console) {
        let in_bounds = 
            pos.x >= 0 && 
            pos.y >= 0 &&
            pos.x < con.width() && 
            pos.y < con.height();
        if in_bounds {
            con.set_default_foreground(self.color);
            con.put_char(pos.x, pos.y, self.char, BackgroundFlag::None);
        }
    }

    pub fn attack(&self, other: &mut Object) -> Message {
        other.take_damage(self.attack);
        let msg = format!(
            "{} attacks {} for {} damage.", 
            self.name, other.name, self.attack);
        Message::new(msg)
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        if self.health <= 0 {
            self.blocking = false;
            self.char = '%';
            self.color = DARK_RED;
            self.alive = false;
        }
    }
}

