use crate::{
    constants::{INFO_TEXT_X, INFO_TEXT_Y},
    draw_text, Vec2, DARKGRAY,
};

#[derive(Debug, PartialEq)]
pub enum Cmd {
    Add { pos: Vec2 },
    Del { pos: Vec2 },
    InitDrag { pos: Vec2 },
    Drag { pos: Vec2 },
    Finish,
    None,
}

pub fn inform_user(msg: &'static str) {
    draw_text(msg, INFO_TEXT_X, INFO_TEXT_Y, 20.0, DARKGRAY);
}

#[derive(Clone)]
pub enum Node {
    InBezierCurve { curve: f32, index: f32 },
    Free { index: f32 },
}

impl Node {
    pub fn new<A>(arg: A) -> Self
    where
        A: Into<Self>,
    {
        arg.into()
    }
}

impl From<f32> for Node {
    fn from(index: f32) -> Self {
        Self::Free { index }
    }
}

impl From<usize> for Node {
    fn from(i: usize) -> Self {
        Self::Free { index: i as f32 }
    }
}

impl From<(usize, usize)> for Node {
    fn from(args: (usize, usize)) -> Self {
        let (curve, index) = args;
        Self::InBezierCurve {
            curve: curve as f32,
            index: index as f32,
        }
    }
}

impl From<&Vec2> for Node {
    fn from(args: &Vec2) -> Self {
        let (curve, index) = (*args).into();
        Self::InBezierCurve { curve, index }
    }
}
