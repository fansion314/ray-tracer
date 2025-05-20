mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vec3::Point;
use std::sync::Arc;

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // World

    let mut world = HittableList::default();

    world.add(Arc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));
    world.add(Arc::new(Sphere::new(Point::new(1.3, 0.5, -2.0), 0.7)));

    // Camera

    let camera = Camera::new(image_width, aspect_ratio);
    if let Err(e) = camera.render(&world) {
        eprintln!("Error: {e}");
    }
}
