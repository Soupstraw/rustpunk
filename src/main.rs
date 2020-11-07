mod rustpunk;

use tcod::input::*;
use tcod::console::*;

use rustpunk::gamestate::GameState;
use rustpunk::object::Action;
use rustpunk::pos::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 50;

struct Tcod {
    root: Root,
    con: Offscreen,
}

fn main() {
    let root = Root::initializer()
        .font("dejavu16x16_gs_tc.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("rustrogue")
        .init();
    let mut state = GameState::new();

    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut tcod = Tcod { root, con };
    tcod::system::set_fps(LIMIT_FPS);
    
    state.update();
    while !tcod.root.window_closed() {
        state.render(&mut tcod.con);
        blit(
            &tcod.con, 
            (0, 0), 
            (tcod.con.width(), tcod.con.height()),
            &mut tcod.root,
            (0, 0),
            1.0,
            1.0
        );
        tcod.root.flush();
        if handle_keys(&mut tcod, &mut state) {
            break;
        }
    }
}

enum Command {
    MoveN,
    MoveE,
    MoveS,
    MoveW,
    MoveNE,
    MoveNW,
    MoveSE,
    MoveSW,
    Wait,
    ExitGame,
}

fn handle_keys(tcod: &mut Tcod, state: &mut GameState) -> bool {
    // Get the last keypress
    let maybe_key = check_for_event(KEY_PRESS);
    // Consume all remaining events (hack, because every keypress generates
    // two events and the flags seem to not have a way to filter those)
    events().last();

    match maybe_key {
        Some((_,Event::Key(key))) => {
            // Check whether to exit the game
            // Handle movement
            let command = match key {
                Key { printable: 'h', .. }         => Some(Command::MoveW),
                Key { code: KeyCode::Left, .. }    => Some(Command::MoveW),
                Key { printable: 'l', .. }         => Some(Command::MoveE),
                Key { code: KeyCode::Right, .. }   => Some(Command::MoveE),
                Key { printable: 'k', .. }         => Some(Command::MoveN),
                Key { code: KeyCode::Up, .. }      => Some(Command::MoveN),
                Key { printable: 'j', .. }         => Some(Command::MoveS),
                Key { code: KeyCode::Down, .. }    => Some(Command::MoveS),
                Key { printable: 'y', .. }         => Some(Command::MoveNW),
                Key { printable: 'u', .. }         => Some(Command::MoveNE),
                Key { printable: 'b', .. }         => Some(Command::MoveSW),
                Key { printable: 'n', .. }         => Some(Command::MoveSE),
                Key { printable: '.', .. }         => Some(Command::Wait),
                Key { code: KeyCode::Escape, .. }  => Some(Command::ExitGame),
                _                                  => None,
            };

            return command.map(|x| handle_command(state, x)).unwrap_or(false);
        }
        _ => return false,
    }

    fn handle_command(state: &mut GameState, command: Command) -> bool {
        match command {
            Command::MoveS => {
                state.player_action(Action::Move(Dir::S));
                state.update();
            }
            Command::MoveN => {
                state.player_action(Action::Move(Dir::N));
                state.update();
            }
            Command::MoveE => {
                state.player_action(Action::Move(Dir::E));
                state.update();
            }
            Command::MoveW => {
                state.player_action(Action::Move(Dir::W));
                state.update();
            }
            Command::MoveNW => {
                state.player_action(Action::Move(Dir::NW));
                state.update();
            }
            Command::MoveNE => {
                state.player_action(Action::Move(Dir::NE));
                state.update();
            }
            Command::MoveSW => {
                state.player_action(Action::Move(Dir::SW));
                state.update();
            }
            Command::MoveSE => {
                state.player_action(Action::Move(Dir::SE));
                state.update();
            }
            Command::Wait => {
                state.player_action(Action::Idle);
                state.update();
            }
            Command::ExitGame => return true,
        };
        false
    }
}

