use std::ops::Index;

use crate::ray::Ray;
use crate::{interval::Interval, vec3::Point};

#[derive(Default)]
pub struct Aabb([Interval; 3]);

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self([x, y, z])
    }

    pub fn from_points(a: &Point, b: &Point) -> Self {
        // Treat the two points a and b as extrema for the bounding box, so we don't require a
        // particular minimum/maximum coordinate order.

        let x = if a[0] <= b[0] {
            Interval::from(a[0], b[0])
        } else {
            Interval::from(b[0], a[0])
        };
        let y = if a[1] <= b[1] {
            Interval::from(a[1], b[1])
        } else {
            Interval::from(b[1], a[1])
        };
        let z = if a[2] <= b[2] {
            Interval::from(a[2], b[2])
        } else {
            Interval::from(b[2], a[2])
        };

        Self::new(x, y, z)
    }

    pub fn from_aabbs(box0: &Aabb, box1: &Aabb) -> Self {
        Self::new(
            Interval::from_intervals(&box0[0], &box1[0]),
            Interval::from_intervals(&box0[1], &box1[1]),
            Interval::from_intervals(&box0[2], &box1[2]),
        )
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = &self[axis];
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.min - ray_orig[axis]) * adinv;
            let t1 = (ax.max - ray_orig[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }
}

impl Index<usize> for Aabb {
    type Output = Interval;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
