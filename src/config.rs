use serde_with::serde_as;
use crate::sphere::Sphere;
use crate::point3d::Point3D;
use serde::{Deserialize, Serialize};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Sky {
    // If provided, the sky will be rendered using the equirectangular
    // projected texture loaded from an image file at this path. Else,
    // a light blue colored sky will be used.
    pub texture: Option<(Vec<u8>, usize, usize, String)>,
}

impl Sky {
    pub fn new_default_sky() -> Sky {
        Sky { texture: None }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: u32,
    pub max_depth: usize,
    pub sky: Option<Sky>,
    pub camera: Camera,
    pub objects: Vec<Sphere>,
}
