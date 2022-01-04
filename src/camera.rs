use serde::{Deserialize, Serialize};
use crate::point3d::Point3D;
use crate::ray::Ray;



#[serde(from = "CameraParams")]
#[derive(Debug, Clone, Deserialize, Serialize, Copy)]
pub struct Camera {
	#[serde(skip_serializing)]
	pub origin: Point3D,
	#[serde(skip_serializing)]
	pub lower_left_corner: Point3D,
	#[serde(skip_serializing)]
	pub focal_length: f64,
	#[serde(skip_serializing)]
	pub horizontal: Point3D,

	#[serde(skip_serializing)]
	pub vertical: Point3D,

	look_from: Point3D,
	look_at: Point3D,
	vup: Point3D,
	vfov: f64,
	aspect: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CameraParams {
	pub look_from: Point3D,
	pub look_at: Point3D,
	pub vup: Point3D,
	pub vfov: f64,
	pub aspect: f64,
}

impl From<CameraParams> for Camera {
	fn from(p: CameraParams) -> Self {
		Camera::new(p.look_from, p.look_at, p.vup, p.vfov, p.aspect)
	}
}

impl Camera {
	pub fn new(
		look_from: Point3D,
		look_at: Point3D,
		vup: Point3D,
		vfov: f64,
		aspect: f64,
	) -> Camera {
		let theta = vfov.to_radians();
		let half_height = (theta / 2.).tan();
		let half_width = aspect * half_height;

		let w = (look_from - look_at).unit_vector();
		let u = vup.cross(&w).unit_vector();
		let v = w.cross(&u);

		let origin = look_from;
		let lower_left_corner = origin - (u * half_width) - (v * half_height) - w;
		let horizontal = u * (2. * half_width) as f64;
		let vertical = v * 2. * half_height;


		Camera {
			origin,
			lower_left_corner,
			focal_length: (look_from - look_at).length(),
			horizontal,
			vertical,
			look_from,
			look_at,
			vup,
			vfov,
			aspect,
		}
	}

	pub fn get_ray(&self, u: f64, v: f64) -> Ray {
		Ray::new(
			self.origin,
			self.lower_left_corner + (self.horizontal * u) + (self.vertical * v) - self.origin,
		)
	}
}