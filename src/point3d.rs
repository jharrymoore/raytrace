use rand::Rng;
use std::ops::{Add, Div, Mul, Neg, Sub};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Copy)]
pub struct Point3D {
	x: f64, 
	y: f64,
	z: f64,
}

impl Point3D {
	pub fn new(x: f64, y: f64, z: f64) -> Point3D {
		Point3D {
			x,
			y,
			z,
		}
	}
	pub fn random(min: f64, max: f64) -> Point3D {
		let mut rng = rand::thread_rng();
		Point3D::new(
			rng.gen_range(min..max),
			rng.gen_range(min..max),
			rng.gen_range(min..max),

		)
	}
	pub fn rand_in_unit_sphere() -> Point3D {
		loop {
			let p = Point3D::random(-1.0, 1.0);
			if p.length_squared() < 1.0 {
				return p;
			}
		}
	}
	pub fn length_squared(&self) -> f64 {
		self.x * self.x + self.y * self.y + self.z * self.z
	}

	pub fn distance(&self, other: &Point3D) -> f64 {
		let dx = self.x - other.x;
		let dy = self.y - other.y;
		let dz = self.z - other.z;
		(dx * dx + dy * dy + dz * dz).sqrt()
	}
}


impl Add for Point3D {
	type Output = Point3D;
	fn add(self, other: Point3D) -> Point3D { 
		Point3D {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}
}

impl Sub for Point3D {
	type Output = Point3D;
	fn sub(self, other: Point3D) -> Point3D { 
		Point3D {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}
}

impl Mul<Point3D> for Point3D {
	type Output = Point3D;
	fn mul(self, other: Point3D) -> Point3D { 
		Point3D {
			x: self.x * other.x,
			y: self.y * other.y,
			z: self.z * other.z,
		}
	}
}

impl Div for Point3D {
	type Output = Point3D;
	fn div(self, other: Point3D) -> Point3D { 
		Point3D {
			x: self.x / other.x,
			y: self.y / other.y,
			z: self.z / other.z,
		}
	}
}