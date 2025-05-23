use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point, Vec3f64};
use std::sync::Arc;

pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Arc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, mat: Arc<dyn Material>) -> Self {
        let radius = radius.max(0.0);
        let rvec = Vec3f64::all(radius);
        Self {
            bbox: AABB::from_points(&(&center - &rvec), &(&center + rvec)),
            center: Ray::new(center, Vec3f64::zero()),
            radius,
            mat,
        }
    }

    pub fn new_moving(center1: Point, center2: Point, radius: f64, mat: Arc<dyn Material>) -> Self {
        let dir = center2 - &center1;
        let center = Ray::new(center1, dir);

        let radius = radius.max(0.0);
        let rvec = Vec3f64::all(radius);
        let box1 = AABB::from_points(&(center.at(0.0) - &rvec), &(center.at(0.0) + &rvec));
        let box2 = AABB::from_points(&(center.at(1.0) - &rvec), &(center.at(1.0) + &rvec));
        let bbox = AABB::from_aabbs(&box1, &box2);

        Self {
            center,
            radius,
            mat,
            bbox,
        }
    }

    fn get_sphere_uv(p: &Point) -> (f64, f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
        use std::f64::consts::PI;

        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(*p.x()) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let current_center = self.center.at(r.time());
        let oc = &current_center - r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (&p - current_center) / self.radius;
        let uv = Self::get_sphere_uv(&outward_normal);
        Some(HitRecord::new(
            r,
            t,
            p,
            outward_normal,
            self.mat.clone(),
            uv,
        ))
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
