use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::{INTERVAL_01, Interval};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point, Vec3f64};
use std::sync::Arc;

pub enum Shape2D {
    Parallelogram,
    Triangle,
    Circle,
    Ellipse,
    Annulus { inner: f64 },
}

type Shape2DFn = Box<dyn Fn(f64, f64) -> Option<(f64, f64)> + Send + Sync>;

impl From<Shape2D> for Shape2DFn {
    fn from(value: Shape2D) -> Self {
        match value {
            Shape2D::Parallelogram => Box::new(|a: f64, b: f64| {
                if INTERVAL_01.contains(a) && INTERVAL_01.contains(b) {
                    Some((a, b))
                } else {
                    None
                }
            }),
            Shape2D::Triangle => Box::new(|a: f64, b: f64| {
                if a > 0.0 && b > 0.0 && a + b < 1.0 {
                    Some((a, b))
                } else {
                    None
                }
            }),
            Shape2D::Circle | Shape2D::Ellipse => Box::new(|a: f64, b: f64| {
                if a * a + b * b < 1.0 {
                    Some((a / 2.0 + 0.5, b / 2.0 + 0.5))
                } else {
                    None
                }
            }),
            Shape2D::Annulus { inner } => Box::new(move |a: f64, b: f64| {
                let center_dist = (a * a + b * b).sqrt();
                if inner < center_dist && center_dist < 1.0 {
                    Some((a / 2.0 + 0.5, b / 2.0 + 0.5))
                } else {
                    None
                }
            }),
        }
    }
}

pub struct Quad {
    q: Point,
    u: Vec3f64,
    v: Vec3f64,
    mat: Arc<dyn Material>,

    w: Vec3f64,
    bbox: AABB,
    normal: Vec3f64,
    d: f64,
    contains_fn: Shape2DFn,
}

impl Quad {
    pub fn new(q: Point, u: Vec3f64, v: Vec3f64, mat: Arc<dyn Material>, shape: Shape2D) -> Self {
        let bbox = {
            let quv = &q + &u + &v;
            let qu = &q + &u;
            let qv = &q + &v;
            match shape {
                Shape2D::Parallelogram => {
                    AABB::from_aabbs(&AABB::from_points(&q, &quv), &AABB::from_points(&qu, &qv))
                }
                Shape2D::Triangle => {
                    AABB::from_aabbs(&AABB::from_points(&q, &qv), &AABB::from_points(&qu, &qv))
                }
                Shape2D::Circle | Shape2D::Ellipse | Shape2D::Annulus { .. } => {
                    AABB::from_points(&quv, &(&q - &u - &v))
                }
            }
        };

        let n = u.cross(&v);
        let w = n.clone() / n.dot(&n);
        let normal = u.cross(&v).into_unit_vector();
        let d = normal.dot(&q);
        Self {
            q,
            u,
            v,
            mat,
            w,
            bbox,
            normal,
            d,
            contains_fn: shape.into(),
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(r.direction());

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return None;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - self.normal.dot(r.origin())) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection = r.at(t);
        let planar_hitpt_vector = &intersection - &self.q;
        let alpha = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vector));

        let uv = (self.contains_fn)(alpha, beta)?;

        Some(HitRecord::new(
            r,
            t,
            intersection,
            self.normal.clone(),
            self.mat.clone(),
            uv,
        ))
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
