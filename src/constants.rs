use macroquad::prelude::const_vec2;

use crate::{Color, Vec2, WHITE};

pub const CIRCLE_RADIUS: f32 = 8.;
pub const CIRCLE_THICKNESS: f32 = 2.;
pub const CIRCLE_COLOR: Color = WHITE;
pub const LINE_THICKNESS: f32 = 1.;
pub const LINE_COLOR: Color = WHITE;
pub const DRAG_MIN_OFFSET: f32 = 1.;
pub const DRAG_MAX_OFFSET: f32 = CIRCLE_RADIUS * 6.;

pub const INFO_TEXT_X: f32 = (1f64 / 2f64) as f32;
pub const INFO_TEXT_Y: f32 = (1f64 / 2f64) as f32;

pub const REL_WIN_CONF_POS: Vec2 = const_vec2!([1./10., 1./10.]);
pub const REL_WIN_CONF_SIZE: Vec2 = const_vec2!([4./10., 5./10.]);

/*
const BTN_X: f32 = (1f64 / 2f64) as f32;
const BTN_Y: f32 = (1f64 / 2f64) as f32;
pub const BTN_DRAW_POS: Vec2 = const_vec2!([BTN_X, BTN_Y]);
pub const BTN_DRAW_COLOR: Color = color_u8!(30, 70, 240, 255);
pub const BTN_DRAW_COLOR_HOVERED: Color = color_u8!(30, 70, 240, 220);
pub const BTN_DRAW_COLOR_CLICKED: Color = color_u8!(30, 70, 240, 150);
 */
