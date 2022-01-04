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
	pub fn length(&self) -> f64 {
		self.length_squared().sqrt()
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
	pub fn near_zero(&self) -> bool {
		self.x.abs() < f64::EPSILON && self.y.abs() < f64::EPSILON && self.z.abs() < f64::EPSILON
	}

	pub fn dot(&self, other: &Point3D) -> f64 {
		self.x * other.x + self.y * other.y + self.z * other.z
	}
	pub fn cross(&self, other: &Point3D) -> Point3D {
		Point3D::new(
			self.y * other.z - self.z * other.y,
			self.z * other.x - self.x * other.z,
			self.x * other.y - self.y * other.x,
		)
	}
	pub fn unit_vector(&self) -> Point3D {
		let length = self.length();
		Point3D::new(self.x / length, self.y / length, self.z / length)
	}
	pub fn x(&self) -> f64 {
		self.x
	}

	pub fn y(&self) -> f64 {
		self.y
	}
	pub fn z(&self) -> f64 {
		self.z
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

impl Mul<f64> for Point3D {
    type Output = Point3D;

    fn mul(self, other: f64) -> Point3D {
        Point3D {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<Point3D> for Point3D {
	type Output = Point3D;
	fn div(self, other: Point3D) -> Point3D { 
		Point3D {
			x: self.x / other.x,
			y: self.y / other.y,
			z: self.z / other.z,
		}
	}
}

impl Div<f64> for Point3D {
    type Output = Point3D;

    fn div(self, other: f64) -> Point3D {
        Point3D {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}


impl Neg for Point3D {
	type Output = Point3D;
	fn neg(self) -> Point3D {
		Point3D {
			x: -self.x,
			y: -self.y,
			z: -self.z,
		}
	}
}