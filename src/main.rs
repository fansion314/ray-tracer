mod aabb;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::vec3::{Point, Vec3f64};
use rand::random_range;
use std::sync::Arc;

fn main() {
    // World

    let mut world = HittableList::default();

    let ground_material = Arc::new(Lambertian::new(Color::all(0.5)));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_range(0.0..1.0);
            let center = Point::new(
                a as f64 + 0.9 * random_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * random_range(0.0..1.0),
            );

            if (&center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = &center + Vec3f64::new(0.0, random_range(0.0..0.5), 0.0);
                    world.add(Arc::new(Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_range(0.0..0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    // Camera

    let camera = {
        let mut c = Camera::default();

        c.aspect_ratio = 16.0 / 9.0;
        c.image_width = 400;
        c.samples_per_pixel = 100;
        c.max_depth = 50;

        c.vfov = 20.0;
        c.lookfrom = Point::new(13.0, 2.0, 3.0);
        c.lookat = Point::new(0.0, 0.0, 0.0);
        c.vup = Vec3f64::new(0.0, 1.0, 0.0);

        c.defocus_angle = 0.6;
        c.focus_dist = 10.0;

        c.with_initialized()
    };

    if let Err(e) = camera.render(&world) {
        eprintln!("Error: {e}");
    }
}
