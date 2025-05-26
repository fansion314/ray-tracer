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

pub struct Magnifier {
    sph0: Sphere,
    sph1: Sphere,
    bbox: AABB,
}

impl Magnifier {
    pub fn new(p: Point, d: Vec3f64, h: f64, mat: Arc<dyn Material>) -> Self {
        let r = &d * (0.5 * (h * h / d.dot(&d) + 1.0));
        let c0 = &p + &d - &r;
        let c1 = &p - &d + &r;
        let r = r.length();
        let sph0 = Sphere::new(c0, r, mat.clone());
        let sph1 = Sphere::new(c1, r, mat);

        let d_len = d.length();
        let mut bbox = AABB::from_points(&Point::new(h, h, d_len), &Point::new(-h, -h, -d_len));
        let yaw = d.x().atan2(*d.z());
        bbox = bbox.rotate::<1>(yaw);
        let pitch = d.y().atan2((d.x().powi(2) + d.z().powi(2)).sqrt());
        bbox = bbox.rotate::<0>(pitch);
        bbox = &bbox + &p;  // not efficient enough

        Self { sph0, sph1, bbox }
    }
}

impl Hittable for Magnifier {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let t_all = Interval::UNIVERSE;
        let rec0 = self.sph0.hit(r, t_all)?;
        let rec1 = self.sph1.hit(r, t_all)?;
        if rec0.t < rec1.t {
            let t_next = Interval::from(rec0.t + 0.0001, f64::INFINITY);
            let rec2 = self.sph0.hit(r, t_next)?;
            if rec2.t < rec1.t {
                None
            } else if ray_t.contains(rec1.t) {
                Some(rec1)
            } else if ray_t.contains(rec2.t) {
                Some(rec2)
            } else {
                None
            }
        } else {
            let t_next = Interval::from(rec1.t + 0.0001, f64::INFINITY);
            let rec2 = self.sph1.hit(r, t_next)?;
            if rec2.t < rec0.t {
                None
            } else if ray_t.contains(rec0.t) {
                Some(rec0)
            } else if ray_t.contains(rec2.t) {
                Some(rec2)
            } else {
                None
            }
        }
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
