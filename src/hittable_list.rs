use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::sync::Arc;

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut result_rec = None;
        let mut closest_so_far = ray_tmax;

        for object in self.objects.iter() {
            if let Some(rec) = object.hit(r, ray_tmin, closest_so_far) {
                closest_so_far = rec.t;
                result_rec = Some(rec);
            };
        }

        result_rec
    }
}
