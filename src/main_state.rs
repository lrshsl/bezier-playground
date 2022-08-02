use crate::{
    constants::*,
    draw_circle, draw_circle_lines,
    utils::{inform_user, Cmd, Node},
    Vec2, settings::BezierSettings,
};

pub struct MainState {
    pub settings: BezierSettings,
    bezier_curves: Vec<QuadraticBezierCurve>,
    points: Vec<Vec2>,
    dragging_target: Option<Node>,
}

impl MainState {
    pub fn new(settings: BezierSettings) -> Self {
        Self {
            settings,
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
                self.dragging_target = self.get_closest_node(&pos).and_then(|p| {
                    let distance = (*self.get_point_from_node(&p)).distance(pos);
                    if distance < DRAG_MAX_OFFSET {
                        Some(p)
                    } else {
                        None
                    }
                })
            }
            Cmd::Drag { pos } => match &self.dragging_target {
                Some(node) => {
                    let node = (*node).clone();
                    *self.get_point_from_node(&node) = pos
                }
                None => {}
            },
            Cmd::Del { pos } => {
                match self.get_closest_point(&pos) {
                    Some(_) => {
                        self.points.retain(|&x| (pos - x).length() > CIRCLE_RADIUS)
                        // TODO: only working for free points
                    }
                    None => {}
                }
            }
            Cmd::Finish => {
                if self.points.len() == 3 {
                    self.new_bezier_curve();
                } else {
                    inform_user("Not enough points");
                }
            }
            Cmd::None => {}
        }
    }

    fn new_bezier_curve(&mut self) {
        self.bezier_curves
            .push(QuadraticBezierCurve::from(&mut self.points));
    }

    fn get_closest_point(&mut self, pos: &Vec2) -> Option<&mut Vec2> {
        let mut cur_min: f32 = f32::MAX;
        let mut tmp_closest: Option<&mut Vec2> = None;
        for curve in self.bezier_curves.iter_mut() {
            for p in curve.points.iter_mut() {
                let d = p.distance(*pos);
                if d < cur_min {
                    cur_min = d;
                    tmp_closest = Some(p);
                }
            }
        }
        for p in self.points.iter_mut() {
            let d = p.distance(*pos);
            if d < cur_min {
                cur_min = d;
                tmp_closest = Some(p);
            }
        }
        tmp_closest
    }

    fn get_point_from_node(&mut self, node: &Node) -> &mut Vec2 {
        match node {
            Node::Free { index } => &mut self.points[*index as usize],
            Node::InBezierCurve { curve, index } => {
                &mut self.bezier_curves[*curve as usize].points[*index as usize]
            }
        }
    }

    fn get_closest_node(&mut self, pos: &Vec2) -> Option<Node> {
        let mut cur_min = f32::MAX;
        let mut tmp_result = None;
        for (curve_i, curve) in self.bezier_curves.iter().enumerate() {
            for (point_i, p) in curve.points.iter().enumerate() {
                let d = p.distance(*pos);
                if d < cur_min {
                    cur_min = d;
                    tmp_result = Some(Node::new((curve_i, point_i)));
                }
            }
        }
        for (point_i, p) in self.points.iter().enumerate() {
            let d = p.distance(*pos);
            if d < cur_min {
                cur_min = d;
                tmp_result = Some(Node::new(point_i));
            }
        }
        tmp_result
    }

    pub fn draw(&self) {
        // Circles
        if self.settings.show_circles {
            for p in self.points.iter() {
                self.draw_point(p);
            }
        }
        // Curves
        for curve in self.bezier_curves.iter() {
            for p in curve.points.iter() {
                if self.settings.show_circles {
                    self.draw_point(p);
                }
                for percent in 1..(self.settings.precision as u16) {
                    let t = f32::from(percent) / f32::from(self.settings.precision);
                    let point = (1. - t) * (1. -t) * curve.points[0] + 2. * (t - t*t) * curve.points[1] + t * t * curve.points[2];
                    draw_circle(
                        point.x,
                        point.y,
                        LINE_THICKNESS,
                        LINE_COLOR,
                    );
                }
                /*
                for curve in self.bezier_curves.iter() {
                    for percent in 1..(self.settings.precision + 1) {
                        let t = percent as f32 / self.settings.precision as f32;
                        let f0 = (1. - t) * (1. - t);
                        let f1 = (1. - t) * t;
                        let f2 = t * t;
                        let [p0, p1, p2] = curve.points;
                        let x = (f0 * p0.x) + (2. * f1 * p1.x) + (f2 * p2.x);
                        let y = (f0 * p0.y) + (2. * f1 * p1.y) + (f2 * p2.y);
                        draw_circle(x, y, LINE_THICKNESS, LINE_COLOR)
                    }
                }
                */
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
