mod constants;
mod main_state;
mod utils;

use constants::*;
use main_state::MainState;
use utils::Cmd;

use macroquad::prelude::*;
//use std::{thread, time};

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = MainState::new();
    let mut minman = MouseInputManager::new();

    loop {
        clear_background(BLACK);
        state.exe_cmd(minman.reaction_on_press());

        state.draw();

        if get_fps() < 55 {
            println!("fps: {}", get_fps());
        }
        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "BezierPlayground".to_owned(),
        fullscreen: true,
        high_dpi: true,
        ..Default::default()
    }
}

pub struct MouseInputManager {
    drag_start: Option<Vec2>,
    dragging: bool,
    deleting: bool,
}

impl MouseInputManager {
    fn new() -> Self {
        Self {
            drag_start: None,
            dragging: false,
            deleting: false,
        }
    }

    fn reaction_on_press(&mut self) -> Cmd {
        let pos = Vec2::from(mouse_position());

        if is_mouse_button_pressed(MouseButton::Left) {
            if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
                self.deleting = true;
                Cmd::Del { pos }
            } else {
                self.drag_start = Some(pos);
                Cmd::None
            }
        } else if is_mouse_button_down(MouseButton::Left) {
            if self.dragging {
                Cmd::Drag { pos }
            } else {
                match self.drag_start {
                    Some(drag_startp) => {
                        if drag_startp.distance(pos) > DRAG_MIN_OFFSET {
                            self.dragging = true;
                            Cmd::InitDrag { pos }
                        } else {
                            Cmd::None
                        }
                    }
                    None => Cmd::None,
                }
            }
        } else if is_mouse_button_released(MouseButton::Left) {
            if self.deleting {
                self.deleting = false;
                Cmd::None
            } else if self.dragging {
                self.dragging = false;
                self.drag_start = None;
                Cmd::None
            } else {
                Cmd::Add { pos }
            }
        } else if is_mouse_button_pressed(MouseButton::Right) {
            Cmd::Finish
        } else {
            Cmd::None
        }
    }
}
