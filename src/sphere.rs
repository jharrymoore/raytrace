use std::mem::discriminant;

use palette::float::Float;
use serde::{Deserialize, Serialize};
use crate::point3d::Point3D;
use crate::material::Material;
use crate::ray::HitRecord;
use crate::ray::{Hittable, Ray};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Sphere {
	pub center: Point3D,
	pub radius: f64,
	pub material: Material,
}
impl Sphere {
	pub fn new(center: Point3D, radius: f64, material: Material) -> Sphere {
		Sphere {
			center,
			radius,
			material,
		}
	}
}


fn u_v_from_sphere_hit_point(hit_point_on_sphere: Point3D) -> (f64, f64) {
	let n = hit_point_on_sphere.unit_vector();
	let x = n.x();
	let y = n.y();
	let z = n.z();
	let u = (x.atan2(z) / (2. * std::f64::consts::PI)) + 0.5;
	let v = y * 0.5 + 0.5;
	(u,v)
}

impl Hittable for Sphere {
	fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		let oc = ray.origin - self.center;
		let a = ray.direction.length_squared();
		let half_b = oc.dot(&ray.direction);
		//l^2 - r^2
		let c = oc.length_squared() - self.radius * self.radius;
		let discriminant = (half_b * half_b) - (a * c);

		if discriminant >= 0. {
			let sqrtd = discriminant.sqrt();
			let root_a = ((-half_b) - sqrtd) / a;
			let root_b = ((-half_b) + sqrtd) / a;
			for root in [root_a, root_b].iter() {
				if *root < t_max && *root > t_min {
					let p = ray.at(*root);
					let normal = (p - self.center) / self.radius;
					let front_face = ray.direction.dot(&normal) < 0.;

					let (u,v) = u_v_from_sphere_hit_point(p - self.center);

					return Some(HitRecord {
						t: *root,
						point: p,
						normal: if front_face {normal} else {-normal},
						front_face,
						material: &self.material,
						u,
						v,
					});
				}
			}
		}
		None
	}
}
