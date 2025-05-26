use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use std::sync::Arc;

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(objects: &mut [Arc<dyn Hittable>]) -> Self {
        // Build the bounding box of the span of source objects.
        let mut bbox = AABB::empty();
        for object_index in 0..objects.len() {
            bbox = AABB::from_aabbs(&bbox, objects[object_index].bounding_box());
        }

        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => Self::box_compare::<0>,
            1 => Self::box_compare::<1>,
            _ => Self::box_compare::<2>,
        };

        let object_span = objects.len();

        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>) = match object_span {
            1 => {
                let node = objects[0].clone();
                (node.clone(), node)
            }
            2 => {
                let a = objects[0].clone();
                let b = objects[1].clone();
                (a, b)
            }
            _ => {
                objects.sort_by(|a, b| comparator(a.as_ref(), b.as_ref()));

                let mid = object_span / 2;
                let (left_objs, right_objs) = objects.split_at_mut(mid);
                let left_node = Arc::new(BVHNode::new(left_objs));
                let right_node = Arc::new(BVHNode::new(right_objs));
                (left_node, right_node)
            }
        };

        Self { left, right, bbox }
    }

    fn box_compare<const AXIS: usize>(a: &dyn Hittable, b: &dyn Hittable) -> std::cmp::Ordering {
        a.bounding_box()[AXIS]
            .min
            .total_cmp(&b.bounding_box()[AXIS].min)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, mut ray_t: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, ray_t) {
            return None;
        }

        let hit_left = self.left.hit(r, ray_t);
        let hit_left = if let Some(rec) = hit_left {
            ray_t.max = rec.t;
            Some(rec)
        } else {
            None
        };
        let hit_right = self.right.hit(r, ray_t);

        hit_right.or(hit_left)
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}

impl From<HittableList> for BVHNode {
    fn from(mut value: HittableList) -> Self {
        Self::new(value.objects.as_mut_slice())
    }
}

impl From<Vec<Arc<dyn Hittable>>> for BVHNode {
    fn from(mut value: Vec<Arc<dyn Hittable>>) -> Self {
        Self::new(value.as_mut_slice())
    }
}
