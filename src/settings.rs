use std::ops::Range;

pub struct BezierSettings {
    pub show_circles: bool,
    pub precision: f32,
    pub prec_range: Range<f32>
}

impl BezierSettings {
    pub fn default() -> Self {
        Self {
            show_circles: true,
            precision: 100.,
            prec_range: 1.0..500.,
        }
    }
}