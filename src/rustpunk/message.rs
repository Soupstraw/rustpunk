use tcod::colors::*;

#[derive(Clone)]
pub struct Message {
    pub text: String,
    pub color: Color,
}

impl Message {
    pub fn new(text: String) -> Self {
        Message {
            text: text,
            color: WHITE,
        }
    }
}
