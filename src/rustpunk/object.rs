use crate::rustpunk::pos::*;
use crate::rustpunk::message::Message;
use crate::rustpunk::gamestate::GameState;

use tcod::colors::*;
use tcod::console::*;

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Idle,
    Move(Dir),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Faction {
    Neutral,
    Enemy,
}

#[derive(Clone, Copy, Debug)]
pub enum Controller {
    Dummy,
    AggressiveAI{last_player_pos: Option<Pos>},
    PlayerController{action: Action},
}

impl Controller {
    pub fn player_controller() -> Self {
        Controller::PlayerController {
            action: Action::Idle,
        }
    }

    pub fn aggressive_ai() -> Self {
        Controller::AggressiveAI {
            last_player_pos: None,
        }
    }

    pub fn next_action(&mut self, obj: &Object ,gs: &GameState) -> Action {
        match self {
            Controller::Dummy => Action::Idle,
            Controller::PlayerController {action} => *action,
            Controller::AggressiveAI {ref mut last_player_pos} => {
                let player = gs.get_player();
                if gs.check_los(obj.pos, player.pos) {
                    *last_player_pos = Some(player.pos);
                }
                match last_player_pos {
                    Some(p) => obj.move_towards(*p),
                    None    => Action::Idle,
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Object<'a> {
    pub pos: Pos,
    pub char: char,
    pub color: Color,
    pub name: &'a str,
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub faction: Faction,
    pub alive: bool,
    pub blocking: bool,
    pub controller: Controller,
}

impl<'a> Object<'a> {
    pub fn new(pos: Pos, char: char, color: Color, name: &'a str) -> Object<'a> {
        Object { 
            pos: pos, 
            char: char, 
            color: color,
            name: name,
            health: 10,
            max_health: 10,
            attack: 1,
            faction: Faction::Neutral,
            alive: true,
            blocking: true,
            controller: Controller::Dummy,
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

    pub fn next_action(&mut self, gs: &GameState) -> Action {
        let s = *self;
        self.controller.next_action(&s, gs)
    }

    pub fn move_towards(&self, pos: Pos) -> Action {
        match self.pos.dir_towards(pos) {
            Some(dir) => Action::Move(dir),
            None      => Action::Idle,
        }
    }
}

