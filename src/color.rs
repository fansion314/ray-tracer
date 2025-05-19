use crate::vec3::Color;
use std::io;
use std::io::Write;

pub fn write_color(mut out: impl Write, pixel_color: &Color) -> io::Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Translate the [0,1] component values to the byte range [0,255].
    let rbyte = (255.999 * r) as u8;
    let gbyte = (255.999 * g) as u8;
    let bbyte = (255.999 * b) as u8;

    // Write out the pixel color components.
    writeln!(out, "{rbyte} {gbyte} {bbyte}\n")
}
