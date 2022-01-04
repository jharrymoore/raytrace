use serde::{Deserialize, Serialize};
use crate::point3d::Point3D;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Sphere {
	pub center: Point3D,
	pub radius: f64,
	pub material: Material,
}
impl Sphere {
	fn new(center: Point3D, radius: f64, material: Material) -> Sphere {
		Sphere {
			center,
			radius,
			material,
		}
	}
}

