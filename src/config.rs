use std::fs::File;
use std::io::BufReader;

use jpeg_decoder::Decoder;
use serde_with::serde_as;
use palette::Srgb;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::camera::Camera;
use crate::material::Glass;
use crate::material::Lambertian;
use crate::material::Material;
use crate::material::Metal;
use crate::point3d::Point3D;
use crate::sphere::Sphere;

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Sky {
    // If provided, the sky will be rendered using the equirectangular
    // projected texture loaded from an image file at this path. Else,
    // a light blue colored sky will be used.
	#[serde_as(as = "TextureOptionPixelsAsPath")]
    pub texture: Option<(Vec<u8>, usize, usize, String)>,
}

impl Sky {
    pub fn new_default_sky() -> Sky {
        Sky { texture: None }
    }
}
fn load_texture_image(path: &str) -> (Vec<u8>, usize, usize, String) {
    let file = File::open(path).expect(path);
    let mut decoder = Decoder::new(BufReader::new(file));
    let pixels = decoder.decode().expect("failed to decode image");
    let metadata = decoder.info().unwrap();
    (
        pixels,
        metadata.width as usize,
        metadata.height as usize,
        path.to_string(),
    )
}

serde_with::serde_conv!(
    TextureOptionPixelsAsPath,
    Option<(Vec<u8>, usize, usize, String)>,
    |texture: &Option<(Vec<u8>, usize, usize, String)>| {
        match texture {
            Some(tuple) => tuple.3.clone(),
            None => "".to_string(),
        }
    },
    |value: &str| -> Result<_, std::convert::Infallible> {
        match value {
            "" => Ok(None),
            _ => Ok(Some(load_texture_image(value))),
        }
    }
);
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

fn _make_cover_world() -> Vec<Sphere> {
    let mut world = Vec::new();

    world.push(Sphere::new(
        Point3D::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian(Lambertian::new(Srgb::new(0.5, 0.5, 0.5))),
    ));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Point3D::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if ((center - Point3D::new(4.0, 0.2, 0.0)).length()) < 0.9 {
                continue;
            }

            if choose_mat < 0.8 {
                // diffuse
                world.push(Sphere::new(
                    center,
                    0.2,
                    Material::Lambertian(Lambertian::new(Srgb::new(
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                    ))),
                ));
            } else if choose_mat < 0.95 {
                // metal
                world.push(Sphere::new(
                    center,
                    0.2,
                    Material::Metal(Metal::new(
                        Srgb::new(
                            0.5 * (1.0 + rng.gen::<f32>()),
                            0.5 * (1.0 + rng.gen::<f32>()),
                            0.5 * (1.0 + rng.gen::<f32>()),
                        ),
                        0.5 * rng.gen::<f64>(),
                    )),
                ));
            } else {
                // glass
                world.push(Sphere::new(center, 0.2, Material::Glass(Glass::new(1.5))));
            }
        }
    }

    world.push(Sphere::new(
        Point3D::new(0.0, 1.0, 0.0),
        1.0,
        Material::Glass(Glass::new(1.5)),
    ));
    world.push(Sphere::new(
        Point3D::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambertian(Lambertian::new(Srgb::new(
            0.4 as f32, 0.2 as f32, 0.1 as f32,
        ))),
    ));
    world.push(Sphere::new(
        Point3D::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal(Metal::new(
            Srgb::new(0.7 as f32, 0.6 as f32, 0.5 as f32),
            0.0,
        )),
    ));
    world
}
