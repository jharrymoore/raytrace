use serde::{Serialize, Deserialize}
use palette::Srgb;
pub trait Scatterable {
	fn scatter(&self, &Ray, hit_record: &HitRecord) -> Option<Option<Ray>, Srgb>
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
	fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Srgb)>
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