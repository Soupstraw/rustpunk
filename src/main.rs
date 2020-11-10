mod rustpunk;

use tcod::input::*;
use tcod::console::*;

use rustpunk::view::*;
use rustpunk::gamestate::*;
use rustpunk::object::*;
use rustpunk::pos::*;

const LIMIT_FPS: i32 = 50;

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
                view.render(&mut self.con);
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
                Key { code: KeyCode::Escape, .. }  => Some(Command::CloseView),
                Key { code: KeyCode::Enter, .. }   => Some(Command::Select),
                _                                  => None,
            };

            command.map(|x| self.handle_command(x));
        }
    }

    fn handle_command(&mut self, command: Command){
        let new_command;
        if let Some(ref mut view) = self.view {
            if let Some(c) = view.handle_command(&mut self.state, command) {
                new_command = c;
            } else {
                return;
            }
        } else {
            new_command = command;
        }
        match new_command {
            Command::Move(dir) => self.state.player_action(Action::Move(dir)),
            Command::Wait => self.state.player_action(Action::Idle),
            Command::GetItem => self.open_pickup_menu(),
            Command::CloseView => self.back(),
            Command::OpenInventory => self.open_inventory(),
            Command::ExitGame => self.quit(),
            _ => {}
        }
    }

    fn open_inventory(&mut self) {
        if self.view.is_none() {
            let inv_view = InventoryView::new(&self.state.get_player().inventory);
            self.view = Some(Box::new(inv_view));
        }
    }

    fn open_pickup_menu(&mut self) {
        let invs = self.state.objects_at(self.state.get_player().pos);
        let invs_fil: Vec<&i32> = invs.iter().filter(|x| **x != 0).collect();
        if invs_fil.len() > 0 {
            let other_idx = invs_fil.get(0).expect("There is supposed to be at least one element");
            let pickup_view = PickupView::new(&self.state, **other_idx);
            self.view = Some(Box::new(pickup_view));
        }
    }
    
    fn back(&mut self) {
        self.view = None;
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

