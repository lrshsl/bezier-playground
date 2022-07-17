use macroquad::prelude::const_vec2;

use crate::{Color, Vec2, WHITE};

pub const CIRCLE_RADIUS: f32 = 8.;
pub const CIRCLE_THICKNESS: f32 = 2.;
pub const CIRCLE_COLOR: Color = WHITE;
pub const LINE_THICKNESS: f32 = 1.;
pub const LINE_COLOR: Color = WHITE;
pub const PREC: i32 = 100;
pub const DRAG_MIN_OFFSET: f32 = 1.;
pub const DRAG_MAX_OFFSET: f32 = CIRCLE_RADIUS * 6.;

pub const INFO_TEXT_X: f32 = (1f64 / 10f64) as f32;
pub const INFO_TEXT_Y: f32 = (1f64 / 12f64) as f32;

const BTN_X: f32 = (18f64 / 20f64) as f32;
const BTN_Y: f32 = (18f64 / 20f64) as f32;
const BTN_WIDTH: f32 = (2f64 / 20f64) as f32;
const BTN_HEIGHT: f32 = (2f64 / 20f64) as f32;
pub const BTN_DRAW_POS: Vec2 = const_vec2!([BTN_X, BTN_Y]);
pub const BTN_DRAW_SIZE: Vec2 = const_vec2!([BTN_WIDTH, BTN_HEIGHT]);
