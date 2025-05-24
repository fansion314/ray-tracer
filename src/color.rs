use crate::interval::Interval;
use crate::vec3::{Vec3, Vec3f64};

pub type Color = Vec3f64;
pub type ColorU8 = Vec3<u8>;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

impl From<Color> for ColorU8 {
    fn from(value: Color) -> Self {
        let mut r = *value.x();
        let mut g = *value.y();
        let mut b = *value.z();

        // Apply a linear to gamma transform for gamma 2
        r = linear_to_gamma(r);
        g = linear_to_gamma(g);
        b = linear_to_gamma(b);

        // Translate the [0,1] component values to the byte range [0,255].
        let rbyte = (255.999 * Interval::I01.clamp(r)) as u8;
        let gbyte = (255.999 * Interval::I01.clamp(g)) as u8;
        let bbyte = (255.999 * Interval::I01.clamp(b)) as u8;

        Vec3::new(rbyte, gbyte, bbyte)
    }
}
