use crate::vec3::{Point, Vec3f64};

pub struct Ray {
    orig: Point,
    dir: Vec3f64,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3f64) -> Self {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> &Point {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3f64 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin() + (self.direction() * t)
    }
}
