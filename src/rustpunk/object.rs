use crate::rustpunk::pos::*;
use crate::rustpunk::message::Message;
use crate::rustpunk::gamestate::GameState;

use tcod::colors::*;
use tcod::console::*;
use tcod::random::*;

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

    pub fn next_action(&self, obj: &Object ,gs: &GameState) -> Action {
        match self {
            Controller::Dummy => Action::Idle,
            Controller::PlayerController {action} => *action,
            Controller::AggressiveAI {last_player_pos} => {
                match last_player_pos {
                    Some(p) => obj.move_towards(*p),
                    None    => Action::Idle,
                }
            }
        }
    }

    pub fn update(&mut self, obj: &Object, gs: &GameState) {
        match self {
            Controller::AggressiveAI {ref mut last_player_pos} => {
                let player = gs.get_player();
                if gs.check_los(obj.pos, player.pos) {
                    *last_player_pos = Some(player.pos);
                }
            }
            _ => {}
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct StatBlock {
    str: i32,
    agi: i32,
    con: i32,
}

#[derive(Clone, Copy, Debug)]
pub enum Health {
    HitPoints(i32),
    Invincible,
}

#[derive(Clone, Debug)]
pub struct Object {
    pub pos: Pos,
    pub char: char,
    pub color: Color,
    pub name: &'static str,
    pub health: Health,
    pub faction: Faction,
    pub alive: bool,
    pub blocking: bool,
    pub stat_block: Option<StatBlock>,
    pub controller: Box<Controller>,
}

impl Object {
    pub fn new(pos: Pos, char: char, color: Color, name: &'static str) -> Object {
        Object { 
            pos: pos, 
            char: char, 
            color: color,
            name: name,
            health: Health::HitPoints(10),
            stat_block: Some(StatBlock {
                str: 10,
                agi: 10,
                con: 10,
            }),
            faction: Faction::Neutral,
            alive: true,
            blocking: true,
            controller: Box::new(Controller::Dummy),
        }
    }

    pub fn player(pos: Pos) -> Object {
        let mut o = Object::new(pos, '@', WHITE, "player");
        o.health = Health::HitPoints(o.max_health());
        o.controller = Box::new(Controller::player_controller());
        o
    }

    pub fn wolf(pos: Pos) -> Object {
        let mut o = Object::new(pos, 'w', DARK_GREY, "grey wolf");
        o.health = Health::HitPoints(o.max_health());
        o.controller = Box::new(Controller::aggressive_ai());
        o.stat_block = Some(StatBlock {
            str: 6,
            agi: 8,
            con: 6,
        });
        o
    }

    pub fn max_health(&self) -> i32 {
        let ref stat_block = self.stat_block
            .expect("missing stat block");
        stat_block.con / 2
    }

    pub fn max_damage(&self) -> i32 {
        let stat_block = self.stat_block
            .expect("missing stat block");
        stat_block.str / 2
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

    pub fn update(&mut self, gs: &GameState) {
        let mut controller = *self.controller;
        controller.update(self, gs);
        *self.controller = controller;
    }

    pub fn attack(&self, other: &mut Object) -> Message {
        let stat_block = self.stat_block
            .expect("attacker does not have a stat block");
        let damage = self.roll_damage();
        other.take_damage(damage);
        let msg = format!(
            "{} attacks {} for {} damage.", 
            self.name, other.name, damage);
        Message::new(msg)
    }

    pub fn take_damage(&mut self, damage: i32) {
        match self.health {
            Health::HitPoints(ref mut hp) => {
                *hp -= damage;
                if *hp <= 0 {
                    self.die();
                }
            }
            Invincible => {}
        }
    }

    pub fn die(&mut self) {
        self.blocking = false;
        self.char = '%';
        self.color = DARK_RED;
        self.alive = false;
    }

    pub fn next_action(&self, gs: &GameState) -> Action {
        self.controller.next_action(self, gs)
    }

    pub fn move_towards(&self, pos: Pos) -> Action {
        match self.pos.dir_towards(pos) {
            Some(dir) => Action::Move(dir),
            None      => Action::Idle,
        }
    }

    pub fn roll_damage(&self) -> i32 {
        let rng = Rng::get_instance();
        rng.get_int(1, self.max_damage())
    }
}

