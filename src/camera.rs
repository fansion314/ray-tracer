use crate::color::{write_color, Color};
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point, Vec3f64};
use std::io;
use std::io::Write;

#[derive(Default)]
pub struct Camera {
    image_width: i32,       // Rendered image width in pixel count
    aspect_ratio: f64,      // Ratio of image width over height
    samples_per_pixel: i32, // Count of random samples for each pixel

    image_height: i32,        // Rendered image height
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    center: Point,            // Camera center
    pixel00_loc: Point,       // Location of pixel 0, 0
    pixel_delta_u: Vec3f64,   // Offset to pixel to the right
    pixel_delta_v: Vec3f64,   // Offset to pixel below
}

impl Camera {
    pub fn new(image_width: i32, aspect_ratio: f64, samples_per_pixel: i32) -> Self {
        Self {
            image_width,
            aspect_ratio,
            samples_per_pixel,
            ..Self::default()
        }
        .with_initialized()
    }

    pub fn render(&self, world: &dyn Hittable) -> io::Result<()> {
        let mut stderr = io::stderr();
        let mut stdout = io::stdout();

        // Render

        println!("P3\n{} {} \n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            write!(stderr, "\rScanlines remaining: {} ", self.image_height - j)?;
            stderr.flush()?;

            for i in 0..self.image_width {
                let mut pixel_color = Color::zero();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, world);
                }
                pixel_color *= self.pixel_samples_scale;
                write_color(&mut stdout, &pixel_color)?;
            }
        }

        writeln!(stderr, "\rDone.                 ")?;
        Ok(())
    }

    fn with_initialized(mut self) -> Self {
        // Image

        // Calculate the image height, and ensure that it's at least 1.
        {
            let image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
            self.image_height = if image_height < 1 { 1 } else { image_height };
        }

        // Camera

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = Point::new(0.0, 0.0, 0.0);

        // Determine viewport dimensions.
        let focal_length = 1.0;
        // Viewport widths less than one are ok since they are real valued.
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3f64::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3f64::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = &viewport_u / self.image_width as f64;
        self.pixel_delta_v = &viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = &self.center
            - Vec3f64::new(0.0, 0.0, focal_length)
            - (&viewport_u / 2.0)
            - (&viewport_v / 2.0);
        self.pixel00_loc = &viewport_upper_left + (&self.pixel_delta_u + &self.pixel_delta_v) * 0.5;

        self
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.

        let offset = Self::sample_square();
        let pixel_sample = &self.pixel00_loc
            + &self.pixel_delta_u * (i as f64 + offset.x())
            + &self.pixel_delta_v * (j as f64 + offset.y());

        let ray_origin = self.center.clone();
        let ray_direction = pixel_sample - &ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3f64 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3f64::new(
            rand::random_range(-0.5..0.5),
            rand::random_range(-0.5..0.5),
            0.0,
        )
    }

    fn ray_color(&self, r: &Ray, world: &dyn Hittable) -> Color {
        if let Some(rec) = world.hit(r, Interval::from(0.0, f64::INFINITY)) {
            return (rec.normal + 1.0) * 0.5;
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
