mod constants;
mod main_state;
mod settings;
//mod skin;
mod utils;

use constants::*;
use main_state::MainState;
use settings::BezierSettings;
use utils::Cmd;

use macroquad::{
    prelude::*,
    ui::{root_ui, widgets::Window}, hash,
};
// CodeBlooded

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = MainState::new(
        BezierSettings::default()
    );
    let mut minman = MouseInputManager::new();
    
 //   root_ui().push_skin(&skin);

    loop {
        // Reference for Size
        let sref = screen_width().min(screen_height());
        clear_background(BLACK);

        // Events
        state.exe_cmd(minman.reaction_on_press());

        // Draw Bezier Curves
        state.draw();

        // Draw Ui
        Window::new( hash!(), REL_WIN_CONF_POS * sref, REL_WIN_CONF_SIZE * sref)
            .label("Quick Settings")
            .ui(&mut root_ui(), |ui| {
                ui.tree_node(hash!(), "General", |ui| {
                    ui.checkbox(hash!(), "Show Circles", &mut state.settings.show_circles);
                    ui.slider(hash!(), "Precision", state.settings.prec_range.clone(), &mut state.settings.precision)
                });
            });

/*
        if get_fps() < 55 {
            println!("fps: {}", get_fps());
        }
*/
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
                            self.drag_start = None;
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
