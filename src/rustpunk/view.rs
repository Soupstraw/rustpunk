use core::cmp::*;
use tcod::console::*;

use crate::rustpunk::gamestate::*;
use crate::rustpunk::item::*;
use crate::rustpunk::pos::*;

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
pub const MENU_MARGIN: i32 = 5;

pub enum Command {
    Move(Dir),
    Select,
    GetItem,
    Wait,
    ExitGame,
    CloseView,
    OpenInventory,
}

pub trait View {
    fn handle_command(&mut self, state: &mut GameState, com: Command) -> Option<Command>;
    fn render(&self, con: &mut Offscreen);
}

pub struct InventoryView {
    items: Vec<String>,
    cursor: i32,
}

impl InventoryView {
    pub fn new(inventory: &Inventory) -> Self {
        InventoryView {
            items: inventory.items.iter().map(|x| x.name.clone()).collect(),
            cursor: 0,
        }
    }
}

impl View for InventoryView {
    fn handle_command(&mut self, state: &mut GameState, com: Command) -> Option<Command>{
        match com {
            Command::Move(Dir::N) => self.cursor = max(0, self.cursor-1),
            Command::Move(Dir::S) => self.cursor = min(
                self.items.len() as i32-1, 
                self.cursor-1),
            Command::CloseView => return Some(Command::CloseView),
            _ => {}
        }
        None
    }

    fn render(&self, con: &mut Offscreen) {
        con.print_frame(
            MENU_MARGIN, 
            MENU_MARGIN,
            SCREEN_WIDTH-MENU_MARGIN*2,
            SCREEN_HEIGHT-MENU_MARGIN*2,
            true,
            BackgroundFlag::Set,
            Some("Inventory"));
        for i in 0..self.items.len() {
            con.print(MENU_MARGIN+2, MENU_MARGIN+2+i as i32, &self.items[i]);
        }
    }
}

pub struct PickupView {
    player_items: Vec<String>,
    player_cursor: i32,
    other_items: Vec<String>,
    other_cursor: i32,
    in_player_col: bool,
    active: bool,
}

impl PickupView {
    pub fn new(player_inv: &Inventory, other_inv: &Inventory) -> Self {
        PickupView {
            player_items: player_inv.items.iter().map(|x| x.name.clone()).collect(),
            player_cursor: 0,
            other_items: other_inv.items.iter().map(|x| x.name.clone()).collect(),
            other_cursor: 0,
            in_player_col: false,
            active: true,
        }
    }
}

impl View for PickupView {
    fn handle_command(&mut self, state: &mut GameState, com: Command) -> Option<Command> {
        match com {
            Command::Move(Dir::N) => {
                if self.in_player_col {
                    self.player_cursor = min(self.player_cursor+1, self.player_items.len() as i32-1);
                } else {
                    self.other_cursor = min(self.other_cursor+1, self.other_items.len() as i32-1);
                }
            }
            Command::Move(Dir::S) => {
                if self.in_player_col {
                    self.player_cursor = max(0, self.player_cursor-1);
                } else {
                    self.other_cursor = max(0, self.other_cursor-1);
                }
            }
            Command::Move(Dir::E) => {
                if self.in_player_col {
                    self.in_player_col = false;
                }
            }
            Command::Move(Dir::W) => {
                if !self.in_player_col {
                    self.in_player_col = true;
                }
            }
            Command::CloseView => return Some(Command::CloseView),
            Command::Select => {
            }
            _ => {}
        }
        None
    }

    fn render(&self, con: &mut Offscreen) {
        con.print_frame(
            MENU_MARGIN, 
            MENU_MARGIN,
            SCREEN_WIDTH-MENU_MARGIN*2,
            SCREEN_HEIGHT-MENU_MARGIN*2,
            true,
            BackgroundFlag::Set,
            Some("Get items"));
        for i in 0..self.player_items.len() {
            con.print(MENU_MARGIN+5, MENU_MARGIN+2+i as i32, &self.player_items[i]);
        }
        for i in 0..self.other_items.len() {
            con.print(SCREEN_WIDTH/2+5, MENU_MARGIN+2+i as i32, &self.other_items[i]);
        }
        if self.in_player_col {
            con.put_char(
                MENU_MARGIN+3, 
                MENU_MARGIN+2+self.player_cursor, 
                '>', 
                BackgroundFlag::Set);
        } else {
            con.put_char(
                SCREEN_WIDTH/2+3, 
                MENU_MARGIN+2+self.other_cursor, 
                '>', 
                BackgroundFlag::Set);
        }
    }
}