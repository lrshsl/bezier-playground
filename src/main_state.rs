use crate::{
    constants::*,
    draw_circle, draw_circle_lines,
    utils::{inform_user, Cmd},
    Vec2,
};

pub struct MainState {
    bezier_curves: Vec<QuadraticBezierCurve>,
    points: Vec<Vec2>,
    dragging_target: Option<Vec2>,
}

impl MainState {
    pub fn new() -> Self {
        Self {
            bezier_curves: Vec::new(),
            points: Vec::new(),
            dragging_target: None,
        }
    }

    pub fn exe_cmd(&mut self, cmd: Cmd) {
        match cmd {
            Cmd::Add { pos } => {
                self.points.push(pos);
                if self.points.len() == 3 {
                    self.new_bezier_curve();
                }
            }
            Cmd::InitDrag { pos } => {
                self.dragging_target = match self.get_closest_point(pos) {
                    Some(p) => Some(*p),
                    None => None,
                }
            }
            Cmd::Drag { pos } => match self.get_closest_point(pos) {
                Some(p) => *p = pos,
                None => inform_user("No points to move"),
            },
            _ => {}
        }
    }

    fn new_bezier_curve(&mut self) {
        self.bezier_curves
            .push(QuadraticBezierCurve::from(&mut self.points));
    }

    fn get_closest_point(&mut self, pos: Vec2) -> Option<&mut Vec2> {
        let mut cur_min: f32 = f32::MAX;
        let mut tmp_closest: Option<&mut Vec2> = None;
        for curve in self.bezier_curves.iter_mut() {
            for p in curve.points.iter_mut() {
                let d = p.distance(pos);
                if d < cur_min {
                    cur_min = d;
                    tmp_closest = Some(p);
                }
            }
        }
        for p in self.points.iter_mut() {
            let d = p.distance(pos);
            if d < cur_min {
                cur_min = d;
                tmp_closest = Some(p);
            }
        }
        tmp_closest
    }

    pub fn draw(&self) {
        for p in self.points.iter() {
            self.draw_point(p);
        }
        for bez_curve in self.bezier_curves.iter() {
            for p in bez_curve.points.iter() {
                self.draw_point(p);

                for curve in self.bezier_curves.iter() {
                    for percent in 1..(PREC + 1) {
                        let t = percent as f32 / PREC as f32;
                        let f0 = (1. - t) * (1. - t);
                        let f1 = (1. - t) * t;
                        let f2 = t * t;
                        let [p0, p1, p2] = curve.points;
                        let x = (f0 * p0.x) + (2. * f1 * p1.x) + (f2 * p2.x);
                        let y = (f0 * p0.y) + (2. * f1 * p1.y) + (f2 * p2.y);
                        draw_circle(x, y, LINE_THICKNESS, LINE_COLOR)
                    }
                }
            }
        }
    }

    fn draw_point(&self, p: &Vec2) {
        draw_circle_lines(p.x, p.y, CIRCLE_RADIUS, CIRCLE_THICKNESS, CIRCLE_COLOR);
    }
}

#[derive(Clone)]
struct QuadraticBezierCurve {
    points: [Vec2; 3],
}

impl QuadraticBezierCurve {
    pub fn from(vect: &mut Vec<Vec2>) -> Self {
        assert_eq!(vect.len(), 3);
        let points = [
            vect.pop().unwrap(),
            vect.pop().unwrap(),
            vect.pop().unwrap(),
        ];
        Self { points }
    }
}
