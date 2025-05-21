use crate::color::{Color, write_color};
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::degrees_to_radians;
use crate::vec3::{Point, Vec3f64};
use rayon::prelude::*;
use std::io::{self, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Default)]
pub struct Camera {
    image_width: i32,       // Rendered image width in pixel count
    aspect_ratio: f64,      // Ratio of image width over height
    samples_per_pixel: i32, // Count of random samples for each pixel
    max_depth: i32,         // Maximum number of ray bounces into scene

    vfov: f64, // Vertical view angle (field of view)

    image_height: i32,        // Rendered image height
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    center: Point,            // Camera center
    pixel00_loc: Point,       // Location of pixel 0, 0
    pixel_delta_u: Vec3f64,   // Offset to pixel to the right
    pixel_delta_v: Vec3f64,   // Offset to pixel below
}

impl Camera {
    pub fn new(
        image_width: i32,
        aspect_ratio: f64,
        samples_per_pixel: i32,
        max_depth: i32,
        vfov: f64,
    ) -> Self {
        Self {
            image_width,
            aspect_ratio,
            samples_per_pixel,
            max_depth,
            vfov,
            ..Self::default()
        }
        .with_initialized()
    }

    pub fn render(&self, world: &dyn Hittable) -> io::Result<()> {
        let width = self.image_width as usize;
        let height = self.image_height as usize;
        let samples = self.samples_per_pixel;
        let max_depth = self.max_depth;
        let scale = self.pixel_samples_scale;

        // Allocate a buffer for all pixels
        let mut buffer = vec![Color::zero(); width * height];

        // Shared progress counter
        let counter = Arc::new(AtomicUsize::new(0));
        let mut stderr = io::stderr();

        // Parallel fill: split by rows (j)
        buffer
            .par_chunks_mut(width)
            .enumerate()
            .for_each(|(j, row)| {
                for i in 0..width {
                    let mut col = Color::zero();
                    for _ in 0..samples {
                        let r = self.get_ray(i as u32, j as u32);
                        col += self.ray_color(&r, max_depth, world);
                    }
                    row[i] = col * scale;
                }

                let prev = counter.fetch_add(1, Ordering::SeqCst);
                if prev % 10 == 0 {
                    let mut stderr = stderr.lock();
                    write!(stderr, "\rScanlines remaining: {}    ", height - prev).ok();
                    stderr.flush().ok();
                }
            });

        // Output
        write!(stderr, "\rWriting to file...       ")?;

        let mut stdout = io::stdout();
        writeln!(stdout, "P3\n{} {} \n255\n", width, height)?;

        for j in 0..height {
            for i in 0..width {
                write_color(&mut stdout, &buffer[j * width + i])?;
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

        self.center = Point::zero();

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        // Viewport widths less than one are ok since they are real valued.
        let viewport_height = 2.0 * h * focal_length;
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

    fn get_ray(&self, i: u32, j: u32) -> Ray {
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

    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::zero();
        }

        if let Some(rec) = world.hit(r, Interval::from(0.001, f64::INFINITY)) {
            if let Some((scattered, attenuation)) = rec.mat.scatter(r, &rec) {
                attenuation * self.ray_color(&scattered, depth - 1, world)
            } else {
                Color::zero()
            }
        } else {
            let unit_direction = r.direction().unit_vector();
            let a = 0.5 * (unit_direction.y() + 1.0);
            Color::one() * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
        }
    }
}
