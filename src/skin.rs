

let skin = {
    let button_style = root_ui()
        .style_builder()
        .color(BTN_DRAW_COLOR)
        .color_hovered(BTN_DRAW_COLOR_HOVERED)
        .color_clicked(BTN_DRAW_COLOR_CLICKED)
        .margin(
            RectOffset::new(12., 12., 12., 10.)
        )
        .font_size(
            ((screen_width() / 10.) as u16)
            .max(
                (screen_height() / 10.) as u16
            )
        )
        .build();
    Skin {
        button_style,
        ..root_ui().default_skin()
    }
};