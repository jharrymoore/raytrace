use serde::{Serialize, Deserialize};
use palette::Srgb;
use serde_with::serde_as;
use std::fs::File;
use std::io::BufReader;
use jpeg_decoder::Decoder;
use rand::Rng;

use crate::{ray::{HitRecord, Ray}, point3d::Point3D};

serde_with::serde_conv!(
    SrgbAsArray,
    Srgb,
    |srgb: &Srgb| [srgb.red, srgb.green, srgb.blue],
    |value: [f32; 3]| -> Result<_, std::convert::Infallible> {
        Ok(Srgb::new(value[0], value[1], value[2]))
    }
);

// TODO: replace this with the more elegant implementation in config.rs
serde_with::serde_conv!(
    TexturePixelsAsPath,
    Vec<u8>,
    |_pixels: &Vec<u8>| "/tmp/texture.jpg",
    |value: &str| -> Result<_, std::convert::Infallible> { Ok(load_texture_image(value).0) }
);

pub trait Scatterable {
	fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Srgb)>;
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Material {
	Lambertian(Lambertian),
	Metal(Metal),
	Glass(Glass),
	Texture(Texture),
	Light(Light),
}


impl Scatterable for Material {
	fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Srgb)> {
		match self {
			Material::Lambertian(m) => m.scatter(ray, hit_record),
			Material::Metal(m) => m.scatter(ray, hit_record),
			Material::Glass(m) => m.scatter(ray, hit_record),
			Material::Texture(m) => m.scatter(ray, hit_record),
			Material::Light(m) => m.scatter(ray, hit_record),
		}
	}
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Light {}

impl Light {
	pub fn new() -> Light {
		Light{}
	}
}

impl Scatterable for Light {
	fn scatter(&self, _ray: &Ray, _hit_record: &HitRecord) -> Option<(Option<Ray>, Srgb)> {
		Some((None, Srgb::new(1.0, 1.0, 1.0)))
	}
}

// define behaviour for each material


// general matte surface
#[serde_with::serde_as]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Lambertian {
	#[serde_as(as = "SrgbAsArray")]
	pub albedo: Srgb
}

impl Lambertian {
	pub fn new(albedo: Srgb) -> Lambertian {
		Lambertian{ albedo }
	}
}
impl Scatterable for Lambertian {
	fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Srgb)> {
		// scatter ray in random direction
		let mut scatter_direction = hit_record.normal + Point3D::rand_in_unit_sphere();
		if scatter_direction.near_zero() {
			scatter_direction = hit_record.normal
		}
		let target = hit_record.point + scatter_direction;
		let scattered = Ray::new(hit_record.point, target - hit_record.point);
		let attenuated = self.albedo;
		Some((Some(scattered), attenuated))
	}
}
#[serde_with::serde_as]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Metal {
	#[serde_as(as = "SrgbAsArray")]
	pub albedo: Srgb,
	pub fuzz: f64,
}
impl Metal {
	pub fn new(albedo: Srgb, fuzz: f64) -> Metal {
		Metal{ albedo, fuzz }
	}
}

fn reflect(v: &Point3D, n: &Point3D) -> Point3D {
	*v - *n * (2. * v.dot(n))
}

impl Scatterable for Metal {
	fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Srgb)> {
		let reflected = reflect(&ray.direction, &hit_record.normal);
		let scattered = Ray::new(
			hit_record.point,
			reflected + Point3D::rand_in_unit_sphere() * self.fuzz);
		let attenuated = self.albedo;
		// if not perpendicular
		if scattered.direction.dot(&hit_record.normal) > 0.0 {
			Some((Some(scattered), attenuated))
		} else {
			None
		}
	}
}

#[serde_with::serde_as]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Glass {
	pub idx_of_refraction: f64,
}

impl Glass {
	pub fn new(idx_of_refraction: f64) -> Glass {
		Glass {
			idx_of_refraction
		}
	}
}

fn refract(uv: &Point3D, n: &Point3D, etai_over_etat: f64) -> Point3D {
	let cos_theta = (-*uv).dot(n).min(1.);
	let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
	let r_out_parallel = *n * (-1. * (1. - r_out_perp.length_squared()).abs().sqrt());
	r_out_parallel + r_out_perp
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
	let mut r0 = (1. - ref_idx) / (1. + ref_idx);
	r0 = r0 * r0;
	r0 + (1. - r0) * (1. - cosine).powi(5)
}

impl Scatterable for Glass {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Srgb)> {
        let mut rng = rand::thread_rng();
        let attenuation = Srgb::new(1.0 as f32, 1.0 as f32, 1.0 as f32);
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.idx_of_refraction
        } else {
            self.idx_of_refraction
        };
        let unit_direction = ray.direction.unit_vector();
        let cos_theta = (-unit_direction).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>() {
            let reflected = reflect(&unit_direction, &hit_record.normal);
            let scattered = Ray::new(hit_record.point, reflected);
            Some((Some(scattered), attenuation))
        } else {
            let direction = refract(&unit_direction, &hit_record.normal, refraction_ratio);
            let scattered = Ray::new(hit_record.point, direction);
            Some((Some(scattered), attenuation))
        }
    }
}

#[serde_with::serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Texture {
    #[serde_as(as = "SrgbAsArray")]
    pub albedo: Srgb,
    #[serde_as(as = "TexturePixelsAsPath")]
    pub pixels: Vec<u8>,
    width: u64,
    height: u64,
    h_offset: f64,
}

fn load_texture_image(path: &str) -> (Vec<u8>, u64, u64) {
    let file = File::open(path).expect(path);
    let mut decoder = Decoder::new(BufReader::new(file));
    let pixels = decoder.decode().expect("failed to decode image");
    let metadata = decoder.info().unwrap();
    (pixels, metadata.width as u64, metadata.height as u64)
}

impl Texture {
    pub fn new(albedo: Srgb, texture_path: &str, rot: f64) -> Texture {
        let file = File::open(texture_path).expect("failed to open texture file");
        let mut decoder = Decoder::new(BufReader::new(file));
        let pixels = decoder.decode().expect("failed to decode image");
        let metadata = decoder.info().unwrap();
        Texture {
            albedo,
            pixels,
            width: metadata.width as u64,
            height: metadata.height as u64,
            h_offset: rot,
        }
    }

    pub fn get_albedo(&self, u: f64, v: f64) -> Srgb {
        let mut rot = u + self.h_offset;
        if rot > 1.0 {
            rot = rot - 1.0;
        }
        let uu = rot * (self.width) as f64;
        let vv = (1.0 - v) * (self.height - 1) as f64;
        let base_pixel =
            (3 * ((vv.floor() as u64) * self.width as u64 + (uu.floor() as u64))) as usize;
        let pixel_r = self.pixels[base_pixel];
        let pixel_g = self.pixels[base_pixel + 1];
        let pixel_b = self.pixels[base_pixel + 2];
        Srgb::new(
            pixel_r as f32 / 255.0,
            pixel_g as f32 / 255.0,
            pixel_b as f32 / 255.0,
        )
    }
}

impl Scatterable for Texture {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Srgb)> {
        let mut scatter_direction = hit_record.normal + Point3D::rand_in_unit_sphere();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let target = hit_record.point + scatter_direction;
        let scattered = Ray::new(hit_record.point, target - hit_record.point);
        let attenuation = self.get_albedo(hit_record.u, hit_record.v);
        Some((Some(scattered), attenuation))
    }
}



