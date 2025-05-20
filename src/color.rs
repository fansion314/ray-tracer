use crate::interval::Interval;
use crate::vec3::Vec3f64;
use std::io;
use std::io::Write;

pub type Color = Vec3f64;

static INTENSITY: Interval = Interval::from(0.000, 0.999);

pub fn write_color(out: &mut impl Write, pixel_color: &Color) -> io::Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Translate the [0,1] component values to the byte range [0,255].

    let rbyte = (256.0 * INTENSITY.clamp(*r)) as u8;
    let gbyte = (256.0 * INTENSITY.clamp(*g)) as u8;
    let bbyte = (256.0 * INTENSITY.clamp(*b)) as u8;

    // Write out the pixel color components.
    writeln!(out, "{rbyte} {gbyte} {bbyte}\n")
}
