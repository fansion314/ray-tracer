use crate::aabb::AABB;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::{Isotropic, Material};
use crate::ray::Ray;
use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn from(boundary: Arc<dyn Hittable>, density: f64, albedo: Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::from(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    // The current implementation assumes that the object's boundary shape
    // is convex, which works for boundaries like boxes or spheres, but
    // not for shapes like tori or those with holes.
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut rec1 = self.boundary.hit(r, Interval::UNIVERSE)?;
        let mut rec2 = self
            .boundary
            .hit(r, Interval::from(rec1.t + 0.0001, f64::INFINITY))?;

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rand::random_range(0.0f64..1.0).ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let mut rec = rec1;
        rec.t += hit_distance / ray_length;
        rec.p = r.at(rec.t);
        rec.mat = self.phase_function.clone();
        Some(rec)
    }

    fn bounding_box(&self) -> &AABB {
        self.boundary.bounding_box()
    }
}
