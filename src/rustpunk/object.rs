use crate::rustpunk::pos::*;
use crate::rustpunk::message::Message;
use crate::rustpunk::gamestate::GameState;
use crate::rustpunk::item::*;

use tcod::colors::*;
use tcod::console::*;
use tcod::random::*;

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Idle,
    Move(Dir),
    GetItem(i32),
    DropItem(i32),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Faction {
    Player,
    Wolves,
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

    pub fn next_action(&self, obj: &Character ,_gs: &GameState) -> Action {
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

    pub fn update(&mut self, obj: &Character, gs: &GameState) {
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

#[derive(Clone, Debug)]
pub struct Character {
    pub pos: Pos,
    pub char: char,
    pub color: Color,
    pub name: &'static str,
    pub health: i32,
    pub faction: Faction,
    pub alive: bool,
    pub blocking: bool,
    pub stat_block: StatBlock,
    pub controller: Box<Controller>,
    pub inventory: Inventory,
}

impl Character {
    pub fn new(
        pos: Pos, 
        char: char, 
        color: Color, 
        name: &'static str, 
        faction: Faction) -> Character {

        Character {
            pos: pos, 
            char: char, 
            color: color,
            name: name,
            health: 10,
            stat_block: StatBlock {
                str: 10,
                agi: 10,
                con: 10,
            },
            faction: faction,
            alive: true,
            blocking: true,
            controller: Box::new(Controller::Dummy),
            inventory: Inventory::new(),
        }
    }

    pub fn player(pos: Pos) -> Character {
        let mut o = Character::new(pos, '@', WHITE, "player", Faction::Player);
        o.health = o.max_health();
        o.faction = Faction::Player;
        o.controller = Box::new(Controller::player_controller());
        o
    }

    pub fn wolf(pos: Pos) -> Character {
        let mut o = Character::new(pos, 'w', DARK_GREY, "grey wolf", Faction::Wolves);
        o.health = o.max_health();
        o.controller = Box::new(Controller::aggressive_ai());
        o.stat_block = StatBlock {
            str: 6,
            agi: 8,
            con: 6,
        };
        o
    }

    pub fn max_health(&self) -> i32 {
        self.stat_block.con / 2
    }

    pub fn max_damage(&self) -> i32 {
        self.stat_block.str / 2
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

    pub fn attack(&self, other: &mut Character) -> Message {
        let damage = self.roll_damage();
        other.take_damage(damage);
        let msg = format!(
            "{} attacks {} for {} damage.", 
            self.name, other.name, damage);
        Message::new(msg)
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        if self.health <= 0 {
            self.die();
        }
    }

    pub fn die(&mut self) {
        self.blocking = false;
        self.char = '%';
        self.color = DARK_RED;
        self.alive = false;
        let corpse = Box::new(self.make_corpse());
        self.inventory.add_item(corpse);
    }

    fn make_corpse(&self) -> Item {
        Item::new(format!("corpse of {}", self.name), format!("It's a corpse."))
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

