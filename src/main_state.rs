use super::bezier_backend::BezierBackend;

use crate::{
    constants::*, draw_circle, draw_circle_lines, settings::BezierSettings, utils::Cmd, Vec2,
};

pub struct MainState {
    pub settings: BezierSettings,
    backend: BezierBackend,
}

impl MainState {
    pub fn new(settings: BezierSettings) -> Self {
        Self {
            settings,
            backend: BezierBackend::new(),
        }
    }

    pub fn exe_cmd(&mut self, cmd: Cmd) {
        self.backend.exe_cmd(cmd)
    }

    pub fn draw(&self) {
        // Circles
        if self.settings.show_circles {
            for p in self.backend.points.iter() {
                self.draw_point(p);
            }
        }
        // Curves
        for curve in self.backend.bezier_curves.iter() {
            for p in curve.points.iter() {
                if self.settings.show_circles {
                    self.draw_point(p);
                }
                for percent in 1..(self.settings.precision as u16) {
                    let t = f32::from(percent) / self.settings.precision;
                    let point = (1. - t) * (1. - t) * curve.points[0]
                        + 2. * (t - t * t) * curve.points[1]
                        + t * t * curve.points[2];
                    draw_circle(point.x, point.y, LINE_THICKNESS, LINE_COLOR);
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
