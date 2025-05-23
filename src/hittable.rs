use crate::aabb::AABB;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point, Vec3f64};
use std::sync::Arc;

pub struct HitRecord {
    pub t: f64,
    pub p: Point,
    pub front_face: bool,
    pub normal: Vec3f64,
    pub mat: Arc<dyn Material>,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn new(
        r: &Ray,
        t: f64,
        p: Point,
        outward_normal: Vec3f64,
        mat: Arc<dyn Material>,
        uv: (f64, f64),
    ) -> Self {
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
            mat,
            u: uv.0,
            v: uv.1,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> &AABB;
}
