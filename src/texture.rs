use crate::color::Color;
use crate::interval::Interval;
use crate::perlin::Perlin;
use crate::rtwimage::RtwImage;
use crate::vec3::Point;
use std::sync::Arc;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color;
}

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn from(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point) -> Color {
        self.albedo.clone()
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn from(scale: f64, c1: Color, c2: Color) -> Self {
        Self::new(
            scale,
            Arc::new(SolidColor::from(c1)),
            Arc::new(SolidColor::from(c2)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color {
        let x = (self.inv_scale * p.x()).floor() as i32;
        let y = (self.inv_scale * p.y()).floor() as i32;
        let z = (self.inv_scale * p.z()).floor() as i32;

        if (x + y + z) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    image: RtwImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        Self {
            image: RtwImage::new(filename),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, _p: &Point) -> Color {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.image.no_data() {
            return Color::new(0.0, 1.0, 1.0);
        }

        // Clamp input texture coordinates to [0,1] x [1,0]
        u = Interval::I01.clamp(u);
        v = 1.0 - Interval::I01.clamp(v); // Flip V to image coordinates

        let pixel = self.image.pixel_data(
            (u * self.image.width() as f64) as u32,
            (v * self.image.height() as f64) as u32,
        );

        let color_scale = 1.0 / 255.0;
        Color::new(
            (color_scale * pixel[0] as f64).powi(2),
            (color_scale * pixel[1] as f64).powi(2),
            (color_scale * pixel[2] as f64).powi(2),
        )
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::default(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point) -> Color {
        Color::all(0.5) * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

pub struct StackedPaddedTexture {
    front: Arc<dyn Texture>,
    back: Arc<dyn Texture>,
    i_u: Interval,
    i_v: Interval,
}

impl StackedPaddedTexture {
    pub fn new(
        front: Arc<dyn Texture>,
        back: Arc<dyn Texture>,
        i_u: Interval,
        i_v: Interval,
    ) -> Self {
        Self {
            front,
            back,
            i_u,
            i_v,
        }
    }
}

impl Texture for StackedPaddedTexture {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color {
        if self.i_u.contains(u) && self.i_v.contains(v) {
            self.front.value(
                (u - self.i_u.min) / self.i_u.size(),
                (v - self.i_v.min) / self.i_v.size(),
                p,
            )
        } else {
            self.back.value(u, v, p)
        }
    }
}
