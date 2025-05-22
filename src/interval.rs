#[derive(Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

pub const INTERVAL_EMPTY: Interval = Interval {
    min: f64::INFINITY,
    max: -f64::INFINITY,
};
pub const INTERVAL_UNIVERSE: Interval = Interval {
    min: -f64::INFINITY,
    max: f64::INFINITY,
};

impl Interval {
    pub const fn new() -> Self {
        // Default interval is empty
        Self {
            min: f64::INFINITY,
            max: -f64::INFINITY,
        }
    }

    pub const fn from(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn from_intervals(a: &Self, b: &Self) -> Self {
        // Create the interval tightly enclosing the two input intervals.
        Self::from(a.min.min(b.min), a.max.max(b.max))
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    // pub fn contains(&self, x: f64) -> bool {
    //     self.min <= x && x <= self.max
    // }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub fn into_expand(mut self, delta: f64) -> Self {
        let padding = delta / 2.0;
        self.min = self.min - padding;
        self.max = self.max + padding;
        self
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::new()
    }
}
