use crate::{constants::*, root_ui, screen_height, screen_width, RectOffset};
use macroquad::ui::Skin;

pub struct SkinManager {}

impl SkinManager {
    pub fn get_skin() -> Skin {
        let button_style = root_ui()
            .style_builder()
            .color(BTN_DRAW_COLOR)
            .color_hovered(BTN_DRAW_COLOR_HOVERED)
            .color_clicked(BTN_DRAW_COLOR_CLICKED)
            .margin(RectOffset::new(12., 12., 12., 10.))
            .font_size(((screen_width() / 10.) as u16).max((screen_height() / 10.) as u16))
            .build();

        let label_style = root_ui()
            .style_builder()
            .font_size(((screen_width() / 30.) as u16).min((screen_height() / 30.) as u16))
            .build();

        Skin {
            button_style,
            label_style,
            ..root_ui().default_skin()
        }
    }
}
