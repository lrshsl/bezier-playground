mod constants;
mod main_state;
mod bezier_curve;
mod bezier_backend;
mod settings;
mod skin;
mod utils;

use constants::*;
use main_state::MainState;
use settings::BezierSettings;
use utils::Cmd;

use macroquad::{
    prelude::*,
    ui::root_ui, hash,
};
// CodeBlooded

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = MainState::new(
        BezierSettings::default()
    );
    let mut minman = MouseInputManager::new();
    
    let mut need_update;
    let skin = &skin::SkinManager::get_skin();
    root_ui().push_skin(&skin);

    loop {
        // Reference for Size
        let sref = vec2(screen_width(), screen_height());

        // Events
        let cmd = minman.reaction_on_press();
        need_update = true;
        if cmd == Cmd::None {
            // need_update = false;
        }
        state.exe_cmd(cmd);


        // Draw Bezier Curves
        if !need_update {
            next_frame().await;
            continue;
        }
        // clear_background(BLACK);  // Not neseccary as long as it is BLACK
        state.draw();


        // Draw Ui
        root_ui().window( hash!(), REL_WIN_CONF_POS * sref, REL_WIN_CONF_SIZE * sref, |ui| {
            ui.label(None, "Quick Settings");
                ui.tree_node(hash!(), "General", |ui| {
                    ui.checkbox(hash!(), "Show Circles", &mut state.settings.show_circles);
                    ui.slider(hash!(), "Precision", state.settings.prec_range.clone(), &mut state.settings.precision)
                });
        });

        // if get_fps() < 55 {
        //     println!("fps: {}", get_fps());
        // }

        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "BezierPlayground".to_owned(),
        fullscreen: true,
        high_dpi: HIGH_DPI,
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
        let sref = screen_width().min(screen_height());
        let window_max = (REL_WIN_CONF_POS + REL_WIN_CONF_SIZE) * sref;
        if pos.x < window_max.x && pos.y < window_max.y {
            return Cmd::None
        }

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
