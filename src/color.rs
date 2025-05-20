use crate::interval::Interval;
use crate::vec3::Vec3f64;
use std::io;
use std::io::Write;

pub type Color = Vec3f64;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color(out: &mut impl Write, pixel_color: &Color) -> io::Result<()> {
    let mut r = *pixel_color.x();
    let mut g = *pixel_color.y();
    let mut b = *pixel_color.z();

    // Apply a linear to gamma transform for gamma 2
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // Translate the [0,1] component values to the byte range [0,255].
    static INTENSITY: Interval = Interval::from(0.000, 0.999);
    let rbyte = (256.0 * INTENSITY.clamp(r)) as u8;
    let gbyte = (256.0 * INTENSITY.clamp(g)) as u8;
    let bbyte = (256.0 * INTENSITY.clamp(b)) as u8;

    // Write out the pixel color components.
    writeln!(out, "{rbyte} {gbyte} {bbyte}\n")
}
