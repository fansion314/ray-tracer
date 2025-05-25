use crate::aabb::AABB;
use crate::bvh::BVHNode;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::{Lambertian, Material};
use crate::quad::{Quad, Shape2D};
use crate::ray::Ray;
use crate::texture::{ImageTexture, SubTexture};
use crate::vec3::{Point, Vec3f64};
use std::env;
use std::path::PathBuf;
use std::sync::Arc;

// Assume single object, single texture
pub struct Model {
    bvh: BVHNode,
}

impl Model {
    pub fn new(model_filename: &str, scale: f64) -> Self {
        Self::load(model_filename, None, scale)
    }

    pub fn with_mat(model_filename: &str, mat: Arc<dyn Material>, scale: f64) -> Self {
        Self::load(model_filename, Some(mat), scale)
    }

    fn load(model_filename: &str, mat: Option<Arc<dyn Material>>, scale: f64) -> Self {
        let filename = PathBuf::from(model_filename);
        let modeldir = env::var("RTW_MODELS").ok();

        let search_paths = vec![
            modeldir.map(|dir| PathBuf::from(dir).join(&filename)),
            Some(filename.clone()),
            Some(PathBuf::from("models").join(&filename)),
            Some(PathBuf::from("../models").join(&filename)),
            Some(PathBuf::from("../../models").join(&filename)),
            Some(PathBuf::from("../../../models").join(&filename)),
            Some(PathBuf::from("../../../../models").join(&filename)),
            Some(PathBuf::from("../../../../../models").join(&filename)),
            Some(PathBuf::from("../../../../../../models").join(&filename)),
        ];

        for path in search_paths.into_iter().flatten() {
            if let Some(model) = Self::load_model(&path, mat.clone(), scale) {
                return model;
            }
        }

        panic!("ERROR: Could not load model file '{model_filename}'.");
    }

    fn load_model(path: &PathBuf, mat: Option<Arc<dyn Material>>, scale: f64) -> Option<Self> {
        if !path.exists() {
            return None;
        }

        let (models, materials) = tobj::load_obj(
            &path,
            &tobj::LoadOptions {
                triangulate: true,
                ignore_lines: true,
                ignore_points: true,
                ..Default::default()
            },
        )
        .ok()?;

        let mut default_mat: Arc<dyn Material> = Arc::new(Lambertian::from(Color::all(0.7843)));
        let model_tex = if let Some(mat) = mat {
            default_mat = mat;
            None
        } else {
            if let Ok(mats) = materials {
                let mat = &mats[0];
                if let Some(map_kd) = &mat.diffuse_texture {
                    Some(Arc::new(ImageTexture::new(map_kd)))
                } else {
                    None
                }
            } else {
                None
            }
        };

        if models.len() > 1 {
            eprintln!("Only use the first model and material.");
        }
        let mesh = &models[0].mesh;

        let faces_len = mesh.indices.len() / 3;
        let mut faces: Vec<Arc<dyn Hittable>> = Vec::with_capacity(faces_len);
        for i in 0..faces_len {
            let i_range = (3 * i)..(3 * i + 3);

            let face_normal = if mesh.normal_indices.is_empty() {
                eprintln!("Model has no normals.");
                Vec3f64::new(0.0, 1.0, 0.0)
            } else {
                let normal_face_indices = &mesh.normal_indices[i_range.clone()];
                let mut v = Vec3f64::new(
                    mesh.normals[normal_face_indices[0] as usize] as f64,
                    mesh.normals[normal_face_indices[1] as usize] as f64,
                    mesh.normals[normal_face_indices[2] as usize] as f64,
                );
                if v.near_zero() {
                    v = Vec3f64::new(0.0, 1.0, 0.0);
                }
                v
            };

            let face_indices = &mesh.indices[i_range.clone()];
            let face_pos: Vec<_> = face_indices
                .iter()
                .map(|i| {
                    let i = 3 * *i as usize;
                    Point::new(
                        mesh.positions[i] as f64,
                        mesh.positions[i + 1] as f64,
                        mesh.positions[i + 2] as f64,
                    )
                })
                .collect();

            let mut a = &face_pos[1] - &face_pos[0];
            let mut b = &face_pos[2] - &face_pos[0];
            let should_swap = a.cross(&b).dot(&face_normal) < 0.0;
            if should_swap {
                std::mem::swap(&mut a, &mut b);
            }

            let mat = if model_tex.is_some() && !mesh.texcoord_indices.is_empty() {
                let texcoord_face_indices = &mesh.texcoord_indices[i_range.clone()];

                let get_uv = |i: usize| {
                    let uv_i = 2 * texcoord_face_indices[i] as usize;
                    let mut uv = (mesh.texcoords[uv_i] as f64, mesh.texcoords[uv_i + 1] as f64);
                    if uv.0 < 0.0 {
                        uv.0 += 1.0;
                    }
                    if uv.1 < 0.0 {
                        uv.1 += 1.0;
                    }
                    uv
                };

                let uv0 = get_uv(0);
                let uv1 = get_uv(1);
                let uv2 = get_uv(2);

                let image_tex = model_tex.clone().unwrap();
                let tex = if !should_swap {
                    SubTexture::new(image_tex, uv0, uv1, uv2)
                } else {
                    SubTexture::new(image_tex, uv0, uv2, uv1)
                };
                Arc::new(Lambertian::new(Arc::new(tex)))
            } else {
                default_mat.clone()
            };

            faces.push(Arc::new(Quad::with_shape(
                &face_pos[0] * scale,
                a * scale,
                b * scale,
                mat,
                Shape2D::Triangle,
            )));
        }

        Some(Self {
            bvh: BVHNode::from(faces),
        })
    }
}

impl Hittable for Model {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        self.bvh.hit(r, ray_t)
    }

    fn bounding_box(&self) -> &AABB {
        self.bvh.bounding_box()
    }
}
