use crate::point3d::Point3D;
use crate::material::Material;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
	pub origin: Point3D,
	pub direction: Point3D,
}


impl Ray {
	fn new(origin: Point3D, direction: Point3D) -> Ray {
		Ray{origin, direction}
	}

	pub fn at(&self, t: f64) -> Point3D {
		self.origin + self.direction * t
	}
}

pub struct HitRecord<'Material> {
	pub t: f64,
	pub point: Point3D,
	pub normal: Point3D,
	pub front_face: bool,
	pub material: &'material Material,
	pub u: u64,
	pub v: f64,
}



pub trait Hittable {
	fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

