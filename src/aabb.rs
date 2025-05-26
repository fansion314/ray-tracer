use crate::ray::Ray;
use crate::vec3::{Vec3, Vec3f64};
use crate::{interval::Interval, vec3::Point};
use std::ops::Add;

pub type AABB = Vec3<Interval>;

impl AABB {
    pub fn from_points(a: &Point, b: &Point) -> Self {
        // Treat the two points a and b as extrema for the bounding box, so we don't require a
        // particular minimum/maximum coordinate order.

        let x = if a[0] <= b[0] { a[0]..b[0] } else { b[0]..a[0] }.into();
        let y = if a[1] <= b[1] { a[1]..b[1] } else { b[1]..a[1] }.into();
        let z = if a[2] <= b[2] { a[2]..b[2] } else { b[2]..a[2] }.into();

        Self::new(x, y, z).pad_to_minimums()
    }

    pub fn from_aabbs(box0: &AABB, box1: &AABB) -> Self {
        Self::new(
            Interval::from_intervals(&box0[0], &box1[0]),
            Interval::from_intervals(&box0[1], &box1[1]),
            Interval::from_intervals(&box0[2], &box1[2]),
        )
        .pad_to_minimums()
    }

    fn pad_to_minimums(mut self) -> Self {
        // Adjust the AABB so that no side is narrower than some delta, padding if necessary.

        let delta = 0.0001;
        for i in 0..3 {
            if self[i].size() < delta {
                self[i] = self[i].into_expand(delta)
            }
        }

        self
    }

    pub fn empty() -> Self {
        Self::new(Interval::EMPTY, Interval::EMPTY, Interval::EMPTY)
    }

    pub fn universe() -> Self {
        Self::new(Interval::UNIVERSE, Interval::UNIVERSE, Interval::UNIVERSE)
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

    pub fn longest_axis(&self) -> usize {
        // Returns the index of the longest axis of the bounding box.

        if self[0].size() > self[1].size() {
            if self[0].size() > self[2].size() {
                0
            } else {
                2
            }
        } else {
            if self[1].size() > self[2].size() {
                1
            } else {
                2
            }
        }
    }

    pub fn rotate(&self, axis: usize, angle: f64) -> Self {
        todo!()
    }
}

impl Add<&Vec3f64> for &AABB {
    type Output = AABB;

    fn add(self, rhs: &Vec3f64) -> Self::Output {
        AABB::new(self.x() + rhs[0], self.y() + rhs[1], self.z() + rhs[2])
    }
}
