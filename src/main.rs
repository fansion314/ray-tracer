mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod perlin;
mod quad;
mod ray;
mod rtweekend;
mod rtwimage;
mod sphere;
mod texture;
mod vec3;

use crate::bvh::BVHNode;
use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{Hittable, RotateY, Translate};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::quad::{Quad, Shape2D};
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
        c.background = Color::new(0.70, 0.80, 1.00);

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

    let world = BVHNode::from(&mut world);

    let camera = {
        let mut c = Camera::default();

        c.aspect_ratio = 16.0 / 9.0;
        c.image_width = 800;
        c.samples_per_pixel = 100;
        c.max_depth = 50;
        c.background = Color::new(0.70, 0.80, 1.00);

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
        c.background = Color::new(0.70, 0.80, 1.00);

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

    let pertext = Arc::new(NoiseTexture::new(4.0));
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

    let world = BVHNode::from(&mut world);

    let camera = {
        let mut c = Camera::default();

        c.aspect_ratio = 16.0 / 9.0;
        c.image_width = 1000;
        c.samples_per_pixel = 100;
        c.max_depth = 50;
        c.background = Color::new(0.70, 0.80, 1.00);

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

fn quads() {
    let mut world = HittableList::default();

    // Materials
    let left_red = Arc::new(Lambertian::new(Arc::new(CheckerTexture::from(
        0.5,
        Color::new(1.0, 0.2, 0.2),
        Color::one(),
    ))));
    let back_green = Arc::new(Metal::new(Color::new(0.7, 1.0, 0.7), 0.005));
    let right_blue = Arc::new(Lambertian::new(Arc::new(NoiseTexture::new(0.5))));
    let upper_orange = Arc::new(Lambertian::new(Arc::new(ImageTexture::new("earthmap.jpg"))));
    let lower_teal = Arc::new(Lambertian::from(Color::new(0.2, 0.8, 0.8)));

    // Quads
    world.add(Arc::new(Quad::with_shape(
        Point::new(-3.0, -2.0, 5.0),
        Vec3f64::new(0.0, 0.0, -4.0),
        Vec3f64::new(0.0, 4.0, 0.0),
        left_red,
        Shape2D::Parallelogram,
    )));

    world.add(Arc::new(Quad::with_shape(
        Point::new(-5.0, -5.0, 0.0),
        Vec3f64::new(10.0, 0.0, 0.0),
        Vec3f64::new(0.0, 10.0, 0.0),
        back_green,
        Shape2D::Triangle,
    )));

    world.add(Arc::new(Quad::with_shape(
        Point::new(3.0, 0.0, 3.0),
        Vec3f64::new(0.0, 0.0, 2.0),
        Vec3f64::new(0.0, 2.0, 0.0),
        right_blue,
        Shape2D::Circle,
    )));

    world.add(Arc::new(Quad::with_shape(
        Point::new(0.0, 3.0, 4.0),
        Vec3f64::new(2.0, 0.0, 0.0),
        Vec3f64::new(0.0, 0.0, 6.0),
        upper_orange,
        Shape2D::Ellipse,
    )));

    world.add(Arc::new(Quad::with_shape(
        Point::new(0.0, -3.0, 3.0),
        Vec3f64::new(2.0, 0.0, 0.0),
        Vec3f64::new(0.0, 0.0, -2.0),
        lower_teal,
        Shape2D::Annulus { inner: 0.5 },
    )));

    let world = BVHNode::from(&mut world);

    let camera = {
        let mut c = Camera::default();

        c.aspect_ratio = 1.0;
        c.image_width = 1000;
        c.samples_per_pixel = 100;
        c.max_depth = 50;
        c.background = Color::new(0.70, 0.80, 1.00);

        c.vfov = 80.0;
        c.lookfrom = Point::new(0.0, 0.0, 9.0);
        c.lookat = Point::new(0.0, 0.0, 0.0);
        c.vup = Vec3f64::new(0.0, 1.0, 0.0);

        c.defocus_angle = 0.0;

        c.with_initialized()
    };

    if let Err(e) = camera.render(&world, "quads.png") {
        eprintln!("Error: {e}");
    }
}

fn simple_light() {
    let mut world = HittableList::default();

    let pertext = Arc::new(NoiseTexture::new(4.0));
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

    let difflight = Arc::new(DiffuseLight::from(Color::all(4.0)));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.add(Arc::new(Quad::with_shape(
        Point::new(3.0, 1.0, -2.0),
        Vec3f64::new(2.0, 0.0, 0.0),
        Vec3f64::new(0.0, 2.0, 0.0),
        difflight,
        Shape2D::Parallelogram,
    )));

    let world = BVHNode::from(&mut world);

    let camera = {
        let mut c = Camera::default();

        c.aspect_ratio = 16.0 / 9.0;
        c.image_width = 1600;
        c.samples_per_pixel = 100;
        c.max_depth = 50;
        c.background = Color::zero();

        c.vfov = 20.0;
        c.lookfrom = Point::new(26.0, 3.0, 6.0);
        c.lookat = Point::new(0.0, 2.0, 0.0);
        c.vup = Vec3f64::new(0.0, 1.0, 0.0);

        c.defocus_angle = 0.0;

        c.with_initialized()
    };

    if let Err(e) = camera.render(&world, "light.png") {
        eprintln!("Error: {e}");
    }
}

fn cornell_box() {
    let mut world = HittableList::default();

    let red = Arc::new(Lambertian::from(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from(Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Point::new(555.0, 0.0, 0.0),
        Vec3f64::new(0.0, 555.0, 0.0),
        Vec3f64::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Point::new(0.0, 0.0, 0.0),
        Vec3f64::new(0.0, 555.0, 0.0),
        Vec3f64::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Point::new(343.0, 554.0, 332.0),
        Vec3f64::new(-130.0, 0.0, 0.0),
        Vec3f64::new(0.0, 0.0, -105.0),
        light,
    )));
    world.add(Arc::new(Quad::new(
        Point::new(0.0, 0.0, 0.0),
        Vec3f64::new(555.0, 0.0, 0.0),
        Vec3f64::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point::new(555.0, 555.0, 555.0),
        Vec3f64::new(-555.0, 0.0, 0.0),
        Vec3f64::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point::new(0.0, 0.0, 555.0),
        Vec3f64::new(555.0, 0.0, 0.0),
        Vec3f64::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    {
        let mut box1 = HittableList::default();
        box1.append(Quad::new_box(
            &Point::new(0.0, 0.0, 0.0),
            &Point::new(165.0, 330.0, 165.0),
            white.clone(),
        ));
        let mut box1: Arc<dyn Hittable> = Arc::new(BVHNode::from(&mut box1));
        box1= Arc::new(RotateY::new(box1, 15.0));
        box1 = Arc::new(Translate::new(box1, Vec3f64::new(265.0, 0.0, 295.0)));
        world.add(box1);
    }
    {
        let mut box1 = HittableList::default();
        box1.append(Quad::new_box(
            &Point::new(0.0, 0.0, 0.0),
            &Point::new(165.0, 165.0, 165.0),
            white.clone(),
        ));
        let mut box1: Arc<dyn Hittable> = Arc::new(BVHNode::from(&mut box1));
        box1 = Arc::new(RotateY::new(box1, -18.0));
        box1 = Arc::new(Translate::new(box1, Vec3f64::new(130.0, 0.0, 65.0)));
        world.add(box1);
    }

    let world = BVHNode::from(&mut world);

    let camera = {
        let mut c = Camera::default();

        c.aspect_ratio = 1.0;
        c.image_width = 800;
        c.samples_per_pixel = 100;
        c.max_depth = 50;
        c.background = Color::zero();

        c.vfov = 40.0;
        c.lookfrom = Point::new(278.0, 278.0, -800.0);
        c.lookat = Point::new(278.0, 278.0, 0.0);
        c.vup = Vec3f64::new(0.0, 1.0, 0.0);

        c.defocus_angle = 0.0;

        c.with_initialized()
    };

    if let Err(e) = camera.render(&world, "cornell_box.png") {
        eprintln!("Error: {e}");
    }
}

fn main() {
    match 7 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        _ => {}
    }
}
