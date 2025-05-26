mod aabb;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod model;
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
use crate::constant_medium::ConstantMedium;
use crate::hittable::{Hittable, RotateY, Translate};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::model::Model;
use crate::quad::{Quad, Shape2D};
use crate::sphere::{Magnifier, Sphere};
use crate::texture::{
    CheckerTexture, ImageTexture, NoiseTexture, SolidColor, StackedPaddedTexture,
};
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
                    let albedo = Color::random_range(0.5..1.0);
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

    let world = BVHNode::from(world);

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

    let world = BVHNode::from(world);

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

    let world = BVHNode::from(world);

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

    let world = BVHNode::from(world);

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

    let world = BVHNode::from(world);

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

    let white_texture = Arc::new(SolidColor::from(Color::new(0.73, 0.73, 0.73)));

    let red = Arc::new(Lambertian::from(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(white_texture.clone()));
    let green = Arc::new(Lambertian::from(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from(Color::new(18.0, 18.0, 15.0)));
    let glass = Arc::new(Dielectric::new(1.5));
    let mirror = Arc::new(Metal::new(Color::new(0.831, 0.686, 0.216), 0.01));

    let background1 = Arc::new(Lambertian::new(Arc::new(StackedPaddedTexture::new(
        Arc::new(ImageTexture::new("pic1.jpg")),
        white_texture.clone(),
        (0.00..1.00).into(),
        ((1.00 - 1.00 * 3712.0 / 5568.0)..1.00).into(),
    ))));

    world.add(Arc::new(Quad::new(
        Point::new(555.0, 0.0, 0.0),
        Vec3f64::new(0.0, 0.0, 555.0),
        Vec3f64::new(0.0, 555.0, 0.0),
        green,
    ))); // left
    world.add(Arc::new(Quad::new(
        Point::new(0.0, 0.0, 0.0),
        Vec3f64::new(0.0, 555.0, 0.0),
        Vec3f64::new(0.0, 0.0, 555.0),
        red,
    ))); // right
    world.add(Arc::new(Quad::new(
        Point::new(368.0, 554.0, 365.0),
        Vec3f64::new(-180.0, 0.0, 0.0),
        Vec3f64::new(0.0, 0.0, -170.0),
        light,
    )));
    world.add(Arc::new(Quad::new(
        Point::new(0.0, 0.0, 0.0),
        Vec3f64::new(0.0, 0.0, 555.0),
        Vec3f64::new(555.0, 0.0, 0.0),
        white.clone(),
    ))); // ground
    world.add(Arc::new(Quad::new(
        Point::new(555.0, 555.0, 555.0),
        Vec3f64::new(-555.0, 0.0, 0.0),
        Vec3f64::new(0.0, 0.0, -555.0),
        white.clone(),
    ))); // top
    world.add(Arc::new(Quad::new(
        Point::new(555.0, 0.0, 555.0),
        Vec3f64::new(-555.0, 0.0, 0.0),
        Vec3f64::new(0.0, 555.0, 0.0),
        background1,
    ))); // back

    world.add(Arc::new(Sphere::new(
        Point::new(555.0 * 0.5, 343.0, 555.0 * 0.85),
        30.0,
        glass.clone(),
    )));

    let box1 = {
        let mut b: Arc<dyn Hittable> = Arc::new(BVHNode::from(Quad::new_box(
            &Point::new(0.0, 0.0, 0.0),
            &Point::new(200.0, 200.0, 200.0),
            mirror,
        )));
        b = Arc::new(RotateY::new(b, 25.0));
        Arc::new(Translate::new(b, Vec3f64::new(250.0, 0.0, 270.0)))
    };
    world.add(box1);

    let boundary = Arc::new(Sphere::new(Point::new(156.0, 90.0, 135.0), 90.0, glass));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::from(
        boundary,
        0.010,
        Color::new(0.580, 0.0, 0.827),
    )));

    let world = BVHNode::from(world);

    let camera = {
        let mut c = Camera::default();

        c.aspect_ratio = 1.0;
        c.image_width = 1600;
        c.samples_per_pixel = 10000;
        c.max_depth = 80;
        c.background = Color::zero();

        c.vfov = 40.0;
        c.lookfrom = Point::new(278.0, 278.0, -760.0);
        c.lookat = Point::new(278.0, 278.0, 0.0);
        c.vup = Vec3f64::new(0.0, 1.0, 0.0);

        c.defocus_angle = 0.0;

        c.with_initialized()
    };

    if let Err(e) = camera.render(&world, "cornell_box.png") {
        eprintln!("Error: {e}");
    }
}

fn cornell_smoke() {
    let mut world = HittableList::default();

    let red = Arc::new(Lambertian::from(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from(Color::new(7.0, 7.0, 7.0)));

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
        Point::new(113.0, 554.0, 127.0),
        Vec3f64::new(330.0, 0.0, 0.0),
        Vec3f64::new(0.0, 0.0, 305.0),
        light,
    )));
    world.add(Arc::new(Quad::new(
        Point::new(0.0, 555.0, 0.0),
        Vec3f64::new(555.0, 0.0, 0.0),
        Vec3f64::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point::new(0.0, 0.0, 0.0),
        Vec3f64::new(555.0, 0.0, 0.0),
        Vec3f64::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point::new(0.0, 0.0, 555.0),
        Vec3f64::new(555.0, 0.0, 0.0),
        Vec3f64::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = {
        let b = Arc::new(BVHNode::from(Quad::new_box(
            &Point::new(0.0, 0.0, 0.0),
            &Point::new(165.0, 330.0, 165.0),
            white.clone(),
        )));
        let b = Arc::new(RotateY::new(b, 15.0));
        Arc::new(Translate::new(b, Vec3f64::new(265.0, 0.0, 295.0)))
    };

    let box2 = {
        let b = Arc::new(BVHNode::from(Quad::new_box(
            &Point::new(0.0, 0.0, 0.0),
            &Point::new(165.0, 165.0, 165.0),
            white.clone(),
        )));
        let b = Arc::new(RotateY::new(b, -18.0));
        Arc::new(Translate::new(b, Vec3f64::new(130.0, 0.0, 65.0)))
    };

    world.add(Arc::new(ConstantMedium::from(box1, 0.01, Color::zero())));
    world.add(Arc::new(ConstantMedium::from(box2, 0.01, Color::all(1.0))));

    let world = BVHNode::from(world);

    let camera = {
        let mut c = Camera::default();

        c.aspect_ratio = 1.0;
        c.image_width = 600;
        c.samples_per_pixel = 200;
        c.max_depth = 50;
        c.background = Color::zero();

        c.vfov = 40.0;
        c.lookfrom = Point::new(278.0, 278.0, -800.0);
        c.lookat = Point::new(278.0, 278.0, 0.0);
        c.vup = Vec3f64::new(0.0, 1.0, 0.0);

        c.defocus_angle = 0.0;

        c.with_initialized()
    };

    if let Err(e) = camera.render(&world, "cornell_smoke.png") {
        eprintln!("Error: {e}");
    }
}

pub fn final_scene(image_width: i32, samples_per_pixel: i32, max_depth: i32) {
    let mut world = HittableList::default();

    let ground = Arc::new(Lambertian::from(Color::new(0.48, 0.83, 0.53)));
    let boxes_per_side = 20;
    let mut boxes1: Vec<Arc<dyn Hittable>> =
        Vec::with_capacity((boxes_per_side * boxes_per_side) as usize);
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_range(1.0..101.0);
            let z1 = z0 + w;

            boxes1.extend(Quad::new_box(
                &Point::new(x0, y0, z0),
                &Point::new(x1, y1, z1),
                ground.clone(),
            ))
        }
    }
    let bvh1 = Arc::new(BVHNode::from(boxes1));
    world.add(bvh1);

    let light = Arc::new(DiffuseLight::from(Color::all(7.0)));
    world.add(Arc::new(Quad::new(
        Point::new(123.0, 554.0, 147.0),
        Vec3f64::new(300.0, 0.0, 0.0),
        Vec3f64::new(0.0, 0.0, 265.0),
        light,
    )));

    let center1 = Point::new(400.0, 400.0, 200.0);
    let center2 = &center1 + Vec3f64::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::from(Color::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::new_moving(
        center1,
        center2,
        50.0,
        sphere_material,
    )));

    world.add(Arc::new(Sphere::new(
        Point::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Arc::new(Sphere::new(
        Point::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::from(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));

    let boundary = Arc::new(Sphere::new(
        Point::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::from(
        boundary,
        0.0001,
        Color::one(),
    )));

    let emat = Arc::new(Lambertian::new(Arc::new(ImageTexture::new("earthmap.jpg"))));
    world.add(Arc::new(Sphere::new(
        Point::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));

    let pertext = Arc::new(NoiseTexture::new(0.2));
    world.add(Arc::new(Sphere::new(
        Point::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new(pertext)),
    )));

    let white = Arc::new(Lambertian::from(Color::all(0.73)));
    let ns = 1000;
    let mut boxes2: Vec<Arc<dyn Hittable>> = Vec::with_capacity(ns as usize);
    for _ in 0..ns {
        let rand_point = Point::random_range(0.0..165.0);
        boxes2.push(Arc::new(Sphere::new(rand_point, 10.0, white.clone())));
    }

    let bvh2 = Arc::new(BVHNode::from(boxes2));
    let rotated = Arc::new(RotateY::new(bvh2, 15.0));
    let translated = Arc::new(Translate::new(rotated, Vec3f64::new(-100.0, 270.0, 395.0)));
    world.add(translated);

    let world = BVHNode::from(world);

    let camera = {
        let mut c = Camera::default();
        c.aspect_ratio = 1.0;
        c.image_width = image_width;
        c.samples_per_pixel = samples_per_pixel;
        c.max_depth = max_depth;
        c.background = Color::zero();
        c.vfov = 40.0;
        c.lookfrom = Point::new(478.0, 278.0, -600.0);
        c.lookat = Point::new(278.0, 278.0, 0.0);
        c.vup = Vec3f64::new(0.0, 1.0, 0.0);
        c.defocus_angle = 0.0;
        c.with_initialized()
    };

    if let Err(e) = camera.render(&BVHNode::from(world), "final_scene.png") {
        eprintln!("Error: {e}");
    }
}

fn model_load() {
    let mut world = HittableList::default();

    let white_texture = Arc::new(SolidColor::from(Color::new(0.73, 0.73, 0.73)));

    let red = Arc::new(Lambertian::from(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(white_texture.clone()));
    let green = Arc::new(Lambertian::from(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from(Color::new(18.0, 18.0, 15.0)));
    // let glass = Arc::new(Dielectric::new(1.5));
    let gold = Arc::new(Metal::new(Color::new(0.831, 0.686, 0.216), 0.08));

    world.add(Arc::new(Quad::new(
        Point::new(555.0, 0.0, 0.0),
        Vec3f64::new(0.0, 0.0, 555.0),
        Vec3f64::new(0.0, 555.0, 0.0),
        green,
    ))); // left
    world.add(Arc::new(Quad::new(
        Point::new(0.0, 0.0, 0.0),
        Vec3f64::new(0.0, 555.0, 0.0),
        Vec3f64::new(0.0, 0.0, 555.0),
        red,
    ))); // right
    world.add(Arc::new(Quad::new(
        Point::new(368.0, 554.0, 365.0),
        Vec3f64::new(-180.0, 0.0, 0.0),
        Vec3f64::new(0.0, 0.0, -170.0),
        light,
    )));
    world.add(Arc::new(Quad::new(
        Point::new(0.0, 0.0, 0.0),
        Vec3f64::new(0.0, 0.0, 555.0),
        Vec3f64::new(555.0, 0.0, 0.0),
        white.clone(),
    ))); // ground
    world.add(Arc::new(Quad::new(
        Point::new(555.0, 555.0, 555.0),
        Vec3f64::new(-555.0, 0.0, 0.0),
        Vec3f64::new(0.0, 0.0, -555.0),
        white.clone(),
    ))); // top
    world.add(Arc::new(Quad::new(
        Point::new(555.0, 0.0, 555.0),
        Vec3f64::new(-555.0, 0.0, 0.0),
        Vec3f64::new(0.0, 555.0, 0.0),
        white.clone(),
    ))); // back

    let box1 = {
        let mut b: Arc<dyn Hittable> = Arc::new(Model::with_mat("bunny.obj", gold, 1400.0));
        b = Arc::new(RotateY::new(b, 165.0));
        let y_move = -*&b.bounding_box()[1].min;
        Arc::new(Translate::new(b, Vec3f64::new(130.0, y_move, 200.0)))
    };
    world.add(box1);

    let box2 = {
        let mut b: Arc<dyn Hittable> = Arc::new(Model::new("usagi-chiikawa.obj", 230.0));
        b = Arc::new(RotateY::new(b, 190.0));
        let y_move = -*&b.bounding_box()[1].min;
        Arc::new(Translate::new(b, Vec3f64::new(400.0, y_move, 350.0)))
    };
    world.add(box2);

    let world = BVHNode::from(world);

    let camera = {
        let mut c = Camera::default();

        c.aspect_ratio = 1.0;
        c.image_width = 1600;
        c.samples_per_pixel = 10000;
        c.max_depth = 80;
        c.background = Color::zero();

        c.vfov = 40.0;
        c.lookfrom = Point::new(278.0, 278.0, -760.0);
        c.lookat = Point::new(278.0, 278.0, 0.0);
        c.vup = Vec3f64::new(0.0, 1.0, 0.0);

        c.defocus_angle = 0.0;

        c.with_initialized()
    };

    if let Err(e) = camera.render(&world, "model_load.png") {
        eprintln!("Error: {e}");
    }
}

fn magnifier_simulation() {
    let mut world = HittableList::default();

    let white = Arc::new(Lambertian::from(Color::all(0.7)));
    let glass = Arc::new(Dielectric::new(1.5));
    // let light = Arc::new(DiffuseLight::from(Color::all(18.0)));

    world.add(Arc::new(Quad::new(
        Point::new(0.0, 0.0, -500.0),
        Vec3f64::new(500.0, 0.0, 0.0),
        Vec3f64::new(0.0, 500.0, 0.0),
        white,
    )));
    world.add(Arc::new(Magnifier::new(
        Point::new(50.0, 300.0, -400.0),
        Vec3f64::new(35.0, -10.0, -30.0) * 4.0,
        200.0,
        glass,
    )));
    // world.add(Arc::new(Quad::new(
    //     Point::new(-200.0, 200.0, 0.0),
    //     Vec3f64::new(0.0, 300.0, 0.0),
    //     Vec3f64::new(300.0, 0.0, 60.0),
    //     light,
    // )));

    let camera = {
        let mut c = Camera::default();

        c.aspect_ratio = 1.0;
        c.image_width = 800;
        c.samples_per_pixel = 200;
        c.max_depth = 30;
        c.background = Color::zero();
        c.sunlight_dir = Some(Vec3f64::new(25.0, -10.0, -40.0));

        c.vfov = 30.0;
        c.lookfrom = Point::new(400.0, 278.0, 760.0);
        c.lookat = Point::new(278.0, 278.0, -250.0);
        c.vup = Vec3f64::new(0.0, 1.0, 0.0);

        c.defocus_angle = 0.0;

        c.with_initialized()
    };

    if let Err(e) = camera.render(&world, "magnifier_simulation.png") {
        eprintln!("Error: {e}");
    }
}

fn main() {
    match 11 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        9 => final_scene(1600, 10000, 80),
        10 => model_load(),
        11 => magnifier_simulation(),
        _ => final_scene(400, 250, 4),
    };
}
