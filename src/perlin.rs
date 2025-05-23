use crate::vec3::Point;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    randfloat: [f64; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut randfloat = [0.0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            randfloat[i] = rand::random_range(0.0..1.0);
        }

        Perlin {
            randfloat,
            perm_x: Self::generate_perm(),
            perm_y: Self::generate_perm(),
            perm_z: Self::generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point) -> f64 {
        // 注意：对f64直接进行as usize转换会产生预期外的效果
        let i = (((4.0 * p.x()) as i32) & 255) as usize;
        let j = (((4.0 * p.y()) as i32) & 255) as usize;
        let k = (((4.0 * p.z()) as i32) & 255) as usize;

        self.randfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }

    fn generate_perm() -> [usize; POINT_COUNT] {
        let mut p = [0usize; POINT_COUNT];
        for i in 0..POINT_COUNT {
            p[i] = i;
        }
        Self::permute(&mut p);
        p
    }

    fn permute(p: &mut [usize; POINT_COUNT]) {
        for i in (1..POINT_COUNT).rev() {
            let target = rand::random_range(0..=i);
            p.swap(i, target);
        }
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}
