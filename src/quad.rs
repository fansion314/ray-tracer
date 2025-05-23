use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::{INTERVAL_01, Interval};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point, Vec3f64};
use std::sync::Arc;

pub struct Quad {
    q: Point,
    u: Vec3f64,
    v: Vec3f64,
    mat: Arc<dyn Material>,

    w: Vec3f64,
    bbox: AABB,
    normal: Vec3f64,
    d: f64,
}

impl Quad {
    pub fn new(q: Point, u: Vec3f64, v: Vec3f64, mat: Arc<dyn Material>) -> Self {
        let bbox_diagonal1 = AABB::from_points(&q, &(&q + &u + &v));
        let bbox_diagonal2 = AABB::from_points(&(&q + &u), &(&q + &v));
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
            bbox: AABB::from_aabbs(&bbox_diagonal1, &bbox_diagonal2),
            normal,
            d,
        }
    }

    fn is_interior(a: f64, b: f64) -> Option<(f64, f64)> {
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.
        if INTERVAL_01.contains(a) && INTERVAL_01.contains(b) {
            Some((a, b))
        } else {
            None
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

        let uv = Self::is_interior(alpha, beta)?;

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
