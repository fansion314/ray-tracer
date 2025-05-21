use std::fmt::{Debug, Display, Formatter};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Vec3<T>([T; 3]);
pub type Vec3f64 = Vec3<f64>;
pub type Point = Vec3f64;

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { 0: [x, y, z] }
    }

    pub fn x(&self) -> &T {
        &self[0]
    }
    pub fn y(&self) -> &T {
        &self[1]
    }
    pub fn z(&self) -> &T {
        &self[2]
    }
}

impl Vec3f64 {
    pub fn all(e: f64) -> Self {
        Self::new(e, e, e)
    }

    pub fn zero() -> Self {
        Self::all(0.0)
    }

    pub fn one() -> Self {
        Self::all(1.0)
    }

    pub fn random() -> Self {
        Self::random_range(0.0, 1.0)
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self::new(
            rand::random_range(min..max),
            rand::random_range(min..max),
            rand::random_range(min..max),
        )
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        self[0].abs() < s && self[1].abs() < s && self[2].abs() < s
    }

    pub fn dot(&self, v: &Self) -> f64 {
        self[0] * v[0] + self[1] * v[1] + self[2] * v[2]
    }

    pub fn cross(&self, v: &Self) -> Self {
        Vec3::new(
            self[1] * v[2] - self[2] * v[1],
            self[2] * v[0] - self[0] * v[2],
            self[0] * v[1] - self[1] * v[0],
        )
    }

    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }

    pub fn into_unit_vector(self) -> Self {
        let l = self.length();
        self / l
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let p = Self::random_range(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq < 1.0 {
                return p / lensq;
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3f64) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn reflect(&self, n: &Self) -> Self {
        let v = self;
        v - n * v.dot(n) * 2.0
    }

    pub fn refract(&self, n: &Self, etai_over_etat: f64) -> Self {
        let uv = self;
        let cos_theta = (-uv.dot(n)).min(1.0);
        let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
        let r_out_parallel = -n * (1.0 - r_out_perp.length_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
    }
}

impl<T> Neg for &Vec3<T>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn neg(self) -> Self::Output {
        Vec3::new(-self[0], -self[1], -self[2])
    }
}

impl<T> Neg for Vec3<T>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn neg(mut self) -> Self::Output {
        self.0[0] = -self.0[0];
        self.0[1] = -self.0[1];
        self.0[2] = -self.0[2];
        self
    }
}

impl<T> Add for &Vec3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl<T> Add<Vec3<T>> for &Vec3<T>
where
    T: AddAssign + Copy,
{
    type Output = Vec3<T>;

    fn add(self, mut rhs: Vec3<T>) -> Self::Output {
        rhs.0[0] += self[0];
        rhs.0[1] += self[1];
        rhs.0[2] += self[2];
        rhs
    }
}

impl<T> Add for Vec3<T>
where
    T: AddAssign,
{
    type Output = Vec3<T>;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T> AddAssign for Vec3<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        let [x, y, z] = rhs.0;
        self.0[0] += x;
        self.0[1] += y;
        self.0[2] += z;
    }
}

impl<T> Add<&Vec3<T>> for Vec3<T>
where
    T: AddAssign + Copy,
{
    type Output = Vec3<T>;

    fn add(mut self, rhs: &Vec3<T>) -> Self::Output {
        self.0[0] += rhs[0];
        self.0[1] += rhs[1];
        self.0[2] += rhs[2];
        self
    }
}

impl<T> Add<T> for &Vec3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vec3<T>;
    fn add(self, rhs: T) -> Self::Output {
        Vec3::new(self[0] + rhs, self[1] + rhs, self[2] + rhs)
    }
}

impl<T> Add<T> for Vec3<T>
where
    T: AddAssign + Copy,
{
    type Output = Vec3<T>;
    fn add(mut self, rhs: T) -> Self::Output {
        self.0[0] += rhs;
        self.0[1] += rhs;
        self.0[2] += rhs;
        self
    }
}

impl<T> Sub for &Vec3<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl<T> Sub<Vec3<T>> for &Vec3<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn sub(self, mut rhs: Vec3<T>) -> Self::Output {
        rhs.0[0] = self[0] - rhs[0];
        rhs.0[1] = self[1] - rhs[1];
        rhs.0[2] = self[2] - rhs[2];
        rhs
    }
}

impl<T> Sub for Vec3<T>
where
    T: SubAssign,
{
    type Output = Vec3<T>;

    fn sub(mut self, rhs: Self) -> Self::Output {
        let [x, y, z] = rhs.0;
        self.0[0] -= x;
        self.0[1] -= y;
        self.0[2] -= z;
        self
    }
}

impl<T> Sub<&Vec3<T>> for Vec3<T>
where
    T: SubAssign + Copy,
{
    type Output = Vec3<T>;

    fn sub(mut self, rhs: &Vec3<T>) -> Self::Output {
        self.0[0] -= rhs[0];
        self.0[1] -= rhs[1];
        self.0[2] -= rhs[2];
        self
    }
}

impl<T> Sub<T> for &Vec3<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vec3<T>;
    fn sub(self, rhs: T) -> Self::Output {
        Vec3::new(self[0] - rhs, self[1] - rhs, self[2] - rhs)
    }
}

impl<T> Sub<T> for Vec3<T>
where
    T: SubAssign + Copy,
{
    type Output = Vec3<T>;
    fn sub(mut self, rhs: T) -> Self::Output {
        self.0[0] -= rhs;
        self.0[1] -= rhs;
        self.0[2] -= rhs;
        self
    }
}

// Component-wise multiplication
impl<T> Mul for &Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vec3<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl<T> Mul<Vec3<T>> for &Vec3<T>
where
    T: MulAssign + Copy,
{
    type Output = Vec3<T>;
    fn mul(self, mut rhs: Vec3<T>) -> Self::Output {
        rhs.0[0] *= self[0];
        rhs.0[1] *= self[1];
        rhs.0[2] *= self[2];
        rhs
    }
}

impl<T> Mul for Vec3<T>
where
    T: MulAssign,
{
    type Output = Vec3<T>;
    fn mul(mut self, rhs: Self) -> Self::Output {
        let [x, y, z] = rhs.0;
        self.0[0] *= x;
        self.0[1] *= y;
        self.0[2] *= z;
        self
    }
}

impl<T> Mul<&Vec3<T>> for Vec3<T>
where
    T: MulAssign + Copy,
{
    type Output = Vec3<T>;
    fn mul(mut self, rhs: &Vec3<T>) -> Self::Output {
        self.0[0] *= rhs[0];
        self.0[1] *= rhs[1];
        self.0[2] *= rhs[2];
        self
    }
}

// Scalar multiplication: Vec3<T> * T
impl<T> Mul<T> for &Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vec3<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Vec3::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: MulAssign + Copy,
{
    type Output = Vec3<T>;
    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T> MulAssign<T> for Vec3<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.0[0] *= rhs;
        self.0[1] *= rhs;
        self.0[2] *= rhs;
    }
}

// Scalar division: Vec3<T> / T
impl<T> Div<T> for &Vec3<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vec3<T>;
    fn div(self, rhs: T) -> Self::Output {
        Vec3::new(self[0] / rhs, self[1] / rhs, self[2] / rhs)
    }
}

impl<T> Div<T> for Vec3<T>
where
    T: DivAssign + Copy,
{
    type Output = Vec3<T>;
    fn div(mut self, rhs: T) -> Self::Output {
        self.0[0] /= rhs;
        self.0[1] /= rhs;
        self.0[2] /= rhs;
        self
    }
}
