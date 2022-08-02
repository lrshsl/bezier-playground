mod constants;
mod main_state;
<<<<<<< HEAD
mod settings;
//mod skin;
=======
>>>>>>> 1adf21f301f5e92b51ef1661facf86426e971c2e
mod utils;

use constants::*;
use main_state::MainState;
<<<<<<< HEAD
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
 //   root_ui().push_skin(&skin);

    loop {
        // Reference for Size
        let sref = screen_width().min(screen_height());

        // Events
        let cmd = minman.reaction_on_press();
        need_update = true;
        if cmd == Cmd::None {
            // need_update = false;
        }
        state.exe_cmd(cmd);


        // Draw Bezier Curves
        if need_update {
            clear_background(BLACK);
            state.draw();
        }


        // Draw Ui
        root_ui().window( hash!(), REL_WIN_CONF_POS * sref, REL_WIN_CONF_SIZE * sref, |ui| {
            ui.label(None, "window");
                ui.tree_node(hash!(), "General", |ui| {
                    ui.checkbox(hash!(), "Show Circles", &mut state.settings.show_circles);
                    ui.slider(hash!(), "Precision", state.settings.prec_range.clone(), &mut state.settings.precision)
                });
        });

        if get_fps() < 55 {
            println!("fps: {}", get_fps());
        }

=======
use utils::Cmd;

use macroquad::prelude::*;
use std::{thread, time};

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = MainState::new();
    let mut minman = MouseInputManager::new();

    loop {
        clear_background(BLACK);
        state.exe_cmd(minman.reaction_on_press());

        state.draw();

        let dt = get_frame_time() as u64;
        thread::sleep(time::Duration::from_millis(MAX_DT - dt));
>>>>>>> 1adf21f301f5e92b51ef1661facf86426e971c2e
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
<<<<<<< HEAD
        let sref = screen_width().min(screen_height());
        let window_min = REL_WIN_CONF_POS * sref;
        let window_max = (REL_WIN_CONF_POS + REL_WIN_CONF_SIZE) * sref;
        if pos.x > window_min.x && pos.x < window_max.x
            && pos.y > window_min.y && pos.y < window_max.y {
            return Cmd::None
        }
=======
>>>>>>> 1adf21f301f5e92b51ef1661facf86426e971c2e

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
<<<<<<< HEAD
                        if drag_startp.distance(pos) > DRAG_MIN_OFFSET {
                            self.dragging = true;
                            self.drag_start = None;
=======
                        println!("distance: {}", drag_startp.distance(pos));
                        if drag_startp.distance(pos) > DRAG_MIN_OFFSET {
                            self.dragging = true;
>>>>>>> 1adf21f301f5e92b51ef1661facf86426e971c2e
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
<<<<<<< HEAD
=======
                self.drag_start = None;
>>>>>>> 1adf21f301f5e92b51ef1661facf86426e971c2e
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
