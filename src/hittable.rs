use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point, Vec3f64};

#[derive(Default, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Point,
    pub front_face: bool,
    pub normal: Vec3f64,
}

impl HitRecord {
    pub fn new(r: &Ray, t: f64, p: Point, outward_normal: Vec3f64) -> Self {
        let front_face = r.direction().dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            t,
            p,
            front_face,
            normal,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
