use crate::{Color, WHITE};

pub const FPS: f64 = 60.;
pub const MAX_DT: u64 = (1. / FPS) as u64;

pub const CIRCLE_RADIUS: f32 = 8.;
pub const CIRCLE_THICKNESS: f32 = 2.;
pub const CIRCLE_COLOR: Color = WHITE;
pub const LINE_THICKNESS: f32 = 1.;
pub const LINE_COLOR: Color = WHITE;
pub const PREC: i32 = 100;
pub const DRAG_MIN_OFFSET: f32 = 1.;
pub const DRAG_MAX_OFFSET: f32 = CIRCLE_RADIUS * 3.;
