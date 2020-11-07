mod rustpunk;

use tcod::console::*;

use tcod::input::Key;
use tcod::input::KeyCode as KC;

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
        let exit = handle_keys(&mut tcod, &mut state);
        if exit {
            break;
        }
    }
}

fn handle_keys(tcod: &mut Tcod, state: &mut GameState) -> bool {
    // Get the last keypress
    let key = tcod.root.wait_for_keypress(true);

    // Check whether to exit the game
    if key.code == KC::Escape {
        return true
    }

    // Handle movement
    let action = match key {
        Key { printable: 'h', .. }  => Some(Action::Move(Dir::W)),
        Key { code: KC::Left, .. }  => Some(Action::Move(Dir::W)),
        Key { printable: 'l', .. }  => Some(Action::Move(Dir::E)),
        Key { code: KC::Right, .. } => Some(Action::Move(Dir::E)),
        Key { printable: 'k', .. }  => Some(Action::Move(Dir::N)),
        Key { code: KC::Up, .. }    => Some(Action::Move(Dir::N)),
        Key { printable: 'j', .. }  => Some(Action::Move(Dir::S)),
        Key { code: KC::Down, .. }  => Some(Action::Move(Dir::S)),
        Key { printable: 'y', .. }  => Some(Action::Move(Dir::NW)),
        Key { printable: 'u', .. }  => Some(Action::Move(Dir::NE)),
        Key { printable: 'b', .. }  => Some(Action::Move(Dir::SW)),
        Key { printable: 'n', .. }  => Some(Action::Move(Dir::SE)),
        Key { printable: '.', .. }  => Some(Action::Idle),
        _                           => None,
    };

    match action {
        Some(a) => {
            state.player_action(a);
            state.update();
        }
        None => {}
    }
    false
}

