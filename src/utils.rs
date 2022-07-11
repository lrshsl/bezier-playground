use crate::{draw_text, Vec2, DARKGRAY};

pub enum Cmd {
    Add { pos: Vec2 },
    Del { pos: Vec2 },
    InitDrag { pos: Vec2 },
    Drag { pos: Vec2 },
    None,
}

pub fn inform_user(msg: &'static str) {
    draw_text(msg, 20.0, 20.0, 20.0, DARKGRAY);
}
