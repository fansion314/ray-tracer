use crate::aabb::AABB;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::rtweekend::degrees_to_radians;
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

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3f64,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3f64) -> Self {
        let bbox = object.bounding_box() + &offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Move the ray backwards by the offset
        let offset_r = Ray::with_time(r.origin() - &self.offset, r.direction().clone(), r.time());

        // Determine whether an intersection exists along the offset ray (and if so, where)
        let mut rec = self.object.hit(&offset_r, ray_t)?;

        // Move the intersection point forwards by the offset
        rec.p += &self.offset;

        Some(rec)
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let (sin_theta, cos_theta) = radians.sin_cos();
        let bbox = object.bounding_box();

        let mut min = Point::all(f64::INFINITY);
        let mut max = Point::all(-f64::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x().max + (1 - i) as f64 * bbox.x().min;
                    let y = j as f64 * bbox.y().max + (1 - j) as f64 * bbox.y().min;
                    let z = k as f64 * bbox.z().max + (1 - k) as f64 * bbox.z().min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3f64::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        let bbox = AABB::from_points(&min, &max);
        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }

    fn transform(&self, v: &Vec3f64) -> Vec3f64 {
        Point::new(
            (self.cos_theta * v.x()) - (self.sin_theta * v.z()),
            *v.y(),
            (self.sin_theta * v.x()) + (self.cos_theta * v.z()),
        )
    }

    fn transform_back(&self, v: &Vec3f64) -> Vec3f64 {
        Point::new(
            (self.cos_theta * v.x()) + (self.sin_theta * v.z()),
            *v.y(),
            (-self.sin_theta * v.x()) + (self.cos_theta * v.z()),
        )
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Transform the ray from world space to object space.
        let origin = self.transform(r.origin());
        let direction = self.transform(r.direction());
        let rotated_r = Ray::with_time(origin, direction, r.time());

        // Determine whether an intersection exists in object space (and if so, where).
        let mut rec = self.object.hit(&rotated_r, ray_t)?;

        // Transform the intersection from object space back to world space.
        rec.p = self.transform_back(&rec.p);
        rec.normal = self.transform_back(&rec.normal);

        Some(rec)
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
