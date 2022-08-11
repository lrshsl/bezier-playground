use super::bezier_backend::BezierBackend;

use crate::{
    constants::*, draw_circle, draw_circle_lines, draw_line, settings::BezierSettings, utils::Cmd,
    Vec2,
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
        // TODO: overthink moves vs refs etc
        if self.settings.show_circles {
            for p in self.backend.points.iter() {
                self.draw_point(p)
            }
        }

        if self.settings.show_control_polygon {
            // that's here for readablity
            // TODO: optimize
            for curve in self.backend.bezier_curves.iter() {
                let mut prev = &curve.points[0];
                for p in curve.points[1..].iter() {
                    draw_line(
                        prev.x,
                        prev.y,
                        p.x,
                        p.y,
                        CONTROL_POLYGON_THICKNESS,
                        CONTROL_POLYGON_COLOR,
                    );
                    prev = p;
                }
            }
        }

        // #performancebottleneck
        for curve in self.backend.bezier_curves.iter() {
            for p in curve.points.iter() {
                if self.settings.show_circles {
                    self.draw_point(p)
                }
                for percent in 1..(self.settings.precision as u8) {
                    // TODO (as u8)
                    let t = f32::from(percent) / self.settings.precision;
                    println!("{}", t);
                    let len = curve.points.len() as i32;

                    let mut point = curve.points[0];
                    if len == 3 {
                        // only for performance
                        point *= (1. - t) * (1. - t);
                        point += curve.points[1] * (1. - t) * t * 2.;
                        point += curve.points[2] * t * t;
                    } else {
                        // I still wrote the first 3 iterations by hand. Guess I don't trust the optimizations &)
                        point *= (1. - t).powi(len - 1);
                        point += curve.points[1]
                            * (1. - t).powi(len - 2)
                            * t.powi(1)
                            * PASCALS_TRIANGLE[len as usize][1];
                        point += curve.points[2]
                            * (1. - t).powi(len - 3)
                            * t.powi(2)
                            * PASCALS_TRIANGLE[len as usize][2];
                        for ((i, pt), binominal_coeficient) in curve.points[3..]
                            .iter()
                            .enumerate()
                            .zip(PASCALS_TRIANGLE[len as usize])
                        {
                            // That's for nested 4-loops
                            let i = i as i32 + 2; // P3 -> i = 2
                            let weight = (1. - t).powi(len - i) * t.powi(i); // powi does not give a int result!
                            point += *pt * weight * *binominal_coeficient; // maths + infomatic = * x ** y == not ambiguous.at(all)
                        }
                    }
                    draw_circle(point.x, point.y, LINE_THICKNESS, LINE_COLOR);
                    // (unfinished) Alternative to x.power():
                    // for (itr, pt) in curve.points.iter().enumerate() {
                    //     let weight = 0.;
                    //     for i in 0..itr {
                    //         weight *= 1. - t;
                    //     }
                    //     point += pt * weight;
                    // }

                    // Quadratic:
                    // A    (1 - t)^2      *   t^0
                    // B    (1 - t)^1      *   t^1
                    // C    (1 - t)^0      *   t^2
                }
            }
        }
    }

    fn draw_point(&self, p: &Vec2) {
        draw_circle_lines(p.x, p.y, CIRCLE_RADIUS, CIRCLE_THICKNESS, CIRCLE_COLOR);
    }

    // draw() for quadratic bezier curves only:
    // pub fn draw(&self) {
    //     // Circles
    //     if self.settings.show_circles {
    //         for p in self.backend.points.iter() {
    //             self.draw_point(p);
    //         }
    //     }
    //     // Curves
    //     for curve in self.backend.bezier_curves.iter() {
    //         for p in curve.points.iter() {
    //             if self.settings.show_circles {
    //                 self.draw_point(p);
    //             }
    //             for percent in 1..(self.settings.precision as u16) {
    //                 let t = f32::from(percent) / self.settings.precision;
    //                 let point = (1. - t) * (1. - t) * curve.points[0]
    //                     + 2. * (t - t * t) * curve.points[1]
    //                     + t * t * curve.points[2];
    //                 draw_circle(point.x, point.y, LINE_THICKNESS, LINE_COLOR);
    //             }
    //             /*
    //             for curve in self.bezier_curves.iter() {
    //                 for percent in 1..(self.settings.precision + 1) {
    //                     let t = percent as f32 / self.settings.precision as f32;
    //                     let f0 = (1. - t) * (1. - t);
    //                     let f1 = (1. - t) * t;
    //                     let f2 = t * t;
    //                     let [p0, p1, p2] = curve.points;
    //                     let x = (f0 * p0.x) + (2. * f1 * p1.x) + (f2 * p2.x);
    //                     let y = (f0 * p0.y) + (2. * f1 * p1.y) + (f2 * p2.y);
    //                     draw_circle(x, y, LINE_THICKNESS, LINE_COLOR)
    //                 }
    //             }
    //             */
    //         }
    //     }
    // }
}
