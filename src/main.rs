mod rustpunk;

use core::cmp::*;

use tcod::input::*;
use tcod::console::*;

use rustpunk::item::*;
use rustpunk::gamestate::GameState;
use rustpunk::object::*;
use rustpunk::pos::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const MENU_MARGIN: i32 = 5;
const LIMIT_FPS: i32 = 50;

#[derive(Copy, Clone, Debug, PartialEq)]
enum GuiState {
    Game,
    Inventory,
    GetMenu,
    Menu,
}

enum Command {
    Move(Dir),
    GetItem,
    Wait,
    ExitGame,
    Back,
    OpenInventory,
}

trait View {
    fn handle_command(&mut self, com: Command);
    fn render(&self, con: &mut Offscreen);
    fn is_active(&self) -> bool;
}

struct InventoryView {
    items: Vec<String>,
    cursor: i32,
    active: bool,
}

impl InventoryView {
    pub fn new(inventory: &Inventory) -> Self {
        InventoryView {
            items: inventory.items.iter().map(|x| x.name.clone()).collect(),
            cursor: 0,
            active: true,
        }
    }
}

impl View for InventoryView {
    fn handle_command(&mut self, com: Command) {
        match com {
            Command::Move(Dir::N) => self.cursor = max(0, self.cursor-1),
            Command::Move(Dir::S) => self.cursor = min(
                self.items.len() as i32-1, 
                self.cursor-1),
            Command::Back => self.active = false,
            _ => {}
        }
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

    fn is_active(&self) -> bool {
        self.active
    }
}

struct Game {
    state: GameState,
    root: Root,
    con: Offscreen,
    view: Option<Box<dyn View>>,
    quit: bool,
}

impl Game {
    fn run(&mut self) {
        self.state.update();
        while !(self.root.window_closed() || self.quit) {
            self.handle_keys();
            self.state.render(&mut self.con);
            if let Some(view) = &self.view {
                if view.is_active() {
                    view.render(&mut self.con);
                } else {
                    self.view = None;
                }
            }
            blit(
                &self.con, 
                (0, 0), 
                (self.con.width(), self.con.height()),
                &mut self.root,
                (0, 0),
                1.0,
                1.0
            );
            self.root.flush();
        }
    }

    fn handle_keys(&mut self){
        // Get the last keypress
        let maybe_key = check_for_event(KEY_PRESS);
        // Consume all remaining events (hack, because every keypress generates
        // two events and the flags seem to not have a way to filter those)
        events().last();

        if let Some((_,Event::Key(key))) = maybe_key {
            let command = match key {
                Key { printable: 'h', .. }         => Some(Command::Move(Dir::W)),
                Key { code: KeyCode::Left, .. }    => Some(Command::Move(Dir::W)),
                Key { printable: 'l', .. }         => Some(Command::Move(Dir::E)),
                Key { code: KeyCode::Right, .. }   => Some(Command::Move(Dir::E)),
                Key { printable: 'k', .. }         => Some(Command::Move(Dir::N)),
                Key { code: KeyCode::Up, .. }      => Some(Command::Move(Dir::N)),
                Key { printable: 'j', .. }         => Some(Command::Move(Dir::S)),
                Key { code: KeyCode::Down, .. }    => Some(Command::Move(Dir::S)),
                Key { printable: '.', .. }         => Some(Command::Wait),
                Key { printable: 'g', .. }         => Some(Command::GetItem),
                Key { printable: 'i', .. }         => Some(Command::OpenInventory),
                Key { code: KeyCode::Escape, .. }  => Some(Command::Back),
                _                                  => None,
            };

            command.map(|x| self.handle_command(x));
        }
    }

    fn handle_command(&mut self, command: Command){
        match self.view {
            Some(ref mut view) => {
                view.handle_command(command);
            }
            None => match command {
                Command::Move(dir) => self.state.player_action(Action::Move(dir)),
                Command::Wait => self.state.player_action(Action::Idle),
                Command::GetItem => self.open_pickup_menu(),
                Command::Back => self.back(),
                Command::OpenInventory => self.open_inventory(),
                Command::ExitGame => self.quit(),
            }
        }
    }

    fn open_inventory(&mut self) {
        if self.view.is_none() {
            let inv_view = InventoryView::new(&self.state.get_player().inventory);
            self.view = Some(Box::new(inv_view));
        }
    }

    fn open_pickup_menu(&mut self) {

    }
    
    fn back(&mut self) {

    }

    fn quit(&mut self) {
        self.quit = true;
    }
}

fn main() {
    let root = Root::initializer()
        .font("dejavu16x16_gs_tc.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Vanapagan")
        .init();

    let mut game = Game { 
        root: root, 
        con: Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT),
        state: GameState::new(),
        quit: false,
        view: None,
    };
    tcod::system::set_fps(LIMIT_FPS);
    game.run();
}

