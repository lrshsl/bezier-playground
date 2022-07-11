mod main_state;
use main_state::MainState;

mod utils;
use utils::Cmd;

mod constants;
use constants::*;

use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = MainState::new();
    let mut minman = MouseInputManager::new();

    loop {
        //let dt = get_frame_time();
        clear_background(BLACK);
        state.exe_cmd(minman.reaction_on_press(MouseButton::Left));
        state.exe_cmd(minman.reaction_on_press(MouseButton::Right));

        state.draw();

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
    drag_start: Vec2,
    dragging: bool,
}

impl MouseInputManager {
    fn new() -> Self {
        Self {
            drag_start: Vec2::ZERO,
            dragging: false,
        }
    }

    fn reaction_on_press(&mut self, btn: MouseButton) -> Cmd {
        let pos = Vec2::from(mouse_position());

        match btn {
            MouseButton::Left => {
                if is_mouse_button_pressed(btn) {
                    self.drag_start = pos;
                    Cmd::None
                } else if is_mouse_button_down(btn) {
                    if self.dragging {
                        Cmd::Drag { pos }
                    } else if (self.drag_start - pos).length() > DRAG_MIN_OFFSET {
                        self.dragging = true;
                        Cmd::InitDrag { pos }
                    } else {
                        Cmd::None
                    }
                } else if is_mouse_button_released(btn) {
                    if self.dragging {
                        self.dragging = false;
                        Cmd::None
                    } else {
                        Cmd::Add { pos }
                    }
                } else {
                    Cmd::None
                }
            }
            MouseButton::Right => Cmd::Del { pos },
            _ => Cmd::None,
        }
    }
}
