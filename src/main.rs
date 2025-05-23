mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod perlin;
mod ray;
mod rtweekend;
mod rtwimage;
mod sphere;
mod texture;
mod vec3;

use crate::bvh::BVHNode;
use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::vec3::{Point, Vec3f64};
use rand::random_range;
use std::sync::Arc;

fn bouncing_spheres() {
    // World

    let mut world = HittableList::default();

    let checker = Arc::new(CheckerTexture::from(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let ground_material = Arc::new(Lambertian::new(checker));
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
                    let sphere_material = Arc::new(Lambertian::from(albedo));
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

    let material2 = Arc::new(Lambertian::from(Color::new(0.4, 0.2, 0.1)));
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

    let world = BVHNode::from(&mut world);

    // Camera

    let camera = {
        let mut c = Camera::default();

        c.aspect_ratio = 16.0 / 9.0;
        c.image_width = 1000;
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

    if let Err(e) = camera.render(&world, "image.png") {
        eprintln!("Error: {e}");
    }
}

fn checkered_spheres() {
    let mut world = HittableList::default();

    let checker = Arc::new(CheckerTexture::from(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker)),
    )));

    let camera = {
        let mut c = Camera::default();

        c.aspect_ratio = 16.0 / 9.0;
        c.image_width = 800;
        c.samples_per_pixel = 100;
        c.max_depth = 50;

        c.vfov = 20.0;
        c.lookfrom = Point::new(13.0, 2.0, 3.0);
        c.lookat = Point::new(0.0, 0.0, 0.0);
        c.vup = Vec3f64::new(0.0, 1.0, 0.0);

        c.defocus_angle = 0.0;

        c.with_initialized()
    };

    if let Err(e) = camera.render(&world, "image.png") {
        eprintln!("Error: {e}");
    }
}

fn earth() {
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    let globe = Sphere::new(Point::zero(), 2.0, earth_surface);

    let camera = {
        let mut c = Camera::default();

        c.aspect_ratio = 16.0 / 9.0;
        c.image_width = 1000;
        c.samples_per_pixel = 100;
        c.max_depth = 50;

        c.vfov = 20.0;
        c.lookfrom = Point::new(0.0, 0.0, 12.0);
        c.lookat = Point::new(0.0, 0.0, 0.0);
        c.vup = Vec3f64::new(0.0, 1.0, 0.0);

        c.defocus_angle = 0.0;

        c.with_initialized()
    };

    if let Err(e) = camera.render(&globe, "globe.png") {
        eprintln!("Error: {e}");
    }
}

fn perlin_spheres() {
    let mut world = HittableList::default();

    let pertext = Arc::new(NoiseTexture::default());
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(pertext)),
    )));

    let camera = {
        let mut c = Camera::default();

        c.aspect_ratio = 16.0 / 9.0;
        c.image_width = 1000;
        c.samples_per_pixel = 100;
        c.max_depth = 50;

        c.vfov = 20.0;
        c.lookfrom = Point::new(13.0, 2.0, 3.0);
        c.lookat = Point::new(0.0, 0.0, 0.0);
        c.vup = Vec3f64::new(0.0, 1.0, 0.0);

        c.defocus_angle = 0.0;

        c.with_initialized()
    };

    if let Err(e) = camera.render(&world, "perlin.png") {
        eprintln!("Error: {e}");
    }
}

fn main() {
    match 4 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        _ => {}
    }
}
