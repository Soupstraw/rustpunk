mod rustpunk;

use tcod::console::*;

use tcod::input::Key;
use tcod::input::KeyCode as KC;

use rustpunk::gamestate::GameState;
use rustpunk::pos::Pos;

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
        let player_pos = state.get_player().pos;
        let win_dim = Pos::new(tcod.con.width()/2, tcod.con.height()/2);
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
    let delta = match key {
        Key { printable: 'h', .. } => Some((-1,  0)),
        Key { code: KC::Left, .. } => Some((-1,  0)),
        Key { printable: 'l', .. } => Some(( 1,  0)),
        Key { code: KC::Right, .. } => Some((1,  0)),
        Key { printable: 'k', .. } => Some(( 0, -1)),
        Key { code: KC::Up, .. } => Some((0, -1)),
        Key { printable: 'j', .. } => Some(( 0,  1)),
        Key { code: KC::Down, .. } => Some((0,  1)),
        Key { printable: 'y', .. } => Some((-1, -1)),
        Key { printable: 'u', .. } => Some(( 1, -1)),
        Key { printable: 'b', .. } => Some((-1,  1)),
        Key { printable: 'n', .. } => Some(( 1,  1)),
        Key { printable: '.', .. } => Some(( 0,  0)),
        _                          => None,
    };

    match delta {
        Some((dx, dy)) => {
            state.player_move(Pos::new(dx, dy));
            state.update();
        }
        None => {}
    }
    false
}

