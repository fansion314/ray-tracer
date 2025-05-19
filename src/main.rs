mod color;
mod ray;
mod vec3;

use crate::color::write_color;
use crate::ray::Ray;
use crate::vec3::{Color, Point, Vec3f64};
use std::io;
use std::io::Write;

fn hit_sphere(center: &Point, radius: f64, r: &Ray) -> f64 {
    let oc = center - r.origin();
    let a = r.direction().dot(r.direction());
    let h = r.direction().dot(&oc);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}

fn ray_color(r: &Ray) -> Color {
    let ball_p = Point::new(0.0, 0.0, -1.0);
    let t = hit_sphere(&ball_p, 0.5, r);
    if t > 0.0 {
        let n = (r.at(t) - &ball_p).into_unit_vector();
        return Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
    }

    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() -> io::Result<()> {
    let mut stderr = io::stderr();

    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // Camera

    let focal_length = 1.0;
    // Viewport widths less than one are ok since they are real valued.
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3f64::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3f64::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = &viewport_u / image_width as f64;
    let pixel_delta_v = &viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = &camera_center
        - Vec3f64::new(0.0, 0.0, focal_length)
        - (&viewport_u / 2.0)
        - (&viewport_v / 2.0);
    let pixel00_loc = &viewport_upper_left + (&pixel_delta_u + &pixel_delta_v) * 0.5;

    // Render

    println!("P3\n{image_width} {image_height} \n255\n");

    for j in 0..image_height {
        write!(stderr, "\rScanlines remaining: {} ", image_height - j)?;
        stderr.flush()?;

        for i in 0..image_width {
            let pixel_center =
                &pixel00_loc + (&pixel_delta_u * i as f64) + (&pixel_delta_v * j as f64);
            let ray_direction = pixel_center - &camera_center;
            let r = Ray::new(camera_center.clone(), ray_direction);

            let pixel_color = ray_color(&r);
            write_color(io::stdout(), &pixel_color)?;
        }
    }

    writeln!(stderr, "\rDone.                 ")?;
    Ok(())
}
