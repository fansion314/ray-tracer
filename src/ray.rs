use crate::vec3::{Point, Vec3f64};

pub struct Ray {
    orig: Point,
    dir: Vec3f64,
    tm: f64,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3f64) -> Self {
        Self::with_time(origin, direction, 0.0)
    }

    pub fn with_time(origin: Point, direction: Vec3f64, time: f64) -> Self {
        Self {
            orig: origin,
            dir: direction,
            tm: time,
        }
    }

    pub fn origin(&self) -> &Point {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3f64 {
        &self.dir
    }

    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin() + (self.direction() * t)
    }
}
