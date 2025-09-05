use std::ops::{Add, Div, Mul, Neg, Sub};

const EPSILON: f32 = 0.00001;

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}

impl Tuple {
	pub fn is_vector(&self) -> bool {
		self.w == 0.0
	}

	pub fn is_point(&self) -> bool {
		self.w == 1.0
	}
}

impl PartialEq for Tuple {
	fn eq(&self, rhs: &Tuple) -> bool {
		float_equal(self.x, rhs.x)
			&& float_equal(self.y, rhs.y)
			&& float_equal(self.z, rhs.z)
			&& float_equal(self.w, rhs.w)
	}
}

impl Add for Tuple {
	type Output = Tuple;
	fn add(self, rhs: Self) -> Self::Output {
		Tuple {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
			w: self.w + rhs.w,
		}
	}
}

impl Sub for Tuple {
	type Output = Tuple;
	fn sub(self, rhs: Self) -> Self::Output {
		Tuple {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
			w: self.w - rhs.w,
		}
	}
}

impl Neg for Tuple {
	type Output = Tuple;
	fn neg(self) -> Self::Output {
		Tuple {
			x: -self.x,
			y: -self.y,
			z: -self.z,
			w: -self.w,
		}
	}
}

impl Mul<f32> for Tuple {
	type Output = Tuple;
	fn mul(self, rhs: f32) -> Self::Output {
		Tuple {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
			w: self.w * rhs,
		}
	}
}
impl Mul<i32> for Tuple {
	type Output = Tuple;
	fn mul(self, rhs: i32) -> Self::Output {
		Tuple {
			x: self.x * rhs as f32,
			y: self.y * rhs as f32,
			z: self.z * rhs as f32,
			w: self.w * rhs as f32,
		}
	}
}

impl Div<f32> for Tuple {
	type Output = Tuple;
	fn div(self, rhs: f32) -> Self::Output {
		if rhs == 0.0 {
			panic!("Cannot divide by 0!");
		} else {
			Tuple {
				x: self.x / rhs,
				y: self.y / rhs,
				z: self.z / rhs,
				w: self.w / rhs,
			}
		}
	}
}
impl Div<i32> for Tuple {
	type Output = Tuple;
	fn div(self, rhs: i32) -> Self::Output {
		if rhs == 0 {
			panic!("Cannot divide by 0!");
		} else {
			Tuple {
				x: self.x / rhs as f32,
				y: self.y / rhs as f32,
				z: self.z / rhs as f32,
				w: self.w / rhs as f32,
			}
		}
	}
}

// pub fn new_tuple(x: f32, y: f32, z: f32, w: f32) -> Tuple {
//     Tuple {
//         x: x,
//         y: y,
//         z: z,
//         w: w,
//     }
// }

pub fn new_vector(x: f32, y: f32, z: f32) -> Tuple {
	Tuple {
		x: x,
		y: y,
		z: z,
		w: 0.0,
	}
}

pub fn new_point(x: f32, y: f32, z: f32) -> Tuple {
	Tuple {
		x: x,
		y: y,
		z: z,
		w: 1.0,
	}
}

pub fn float_equal(a: f32, b: f32) -> bool {
	return (a - b).abs() < EPSILON;
}

pub fn magnitude(vector: Tuple) -> f32 {
	(vector.x.powf(2.0) + vector.y.powf(2.0) + vector.z.powf(2.0) + vector.w.powf(2.0)).sqrt()
	// dot_product(vector, vector).sqrt() // is the equivalent
}

pub fn normalize(vector: Tuple) -> Tuple {
	let m = magnitude(vector);
	Tuple {
		x: vector.x / m,
		y: vector.y / m,
		z: vector.z / m,
		w: vector.w / m,
	}
}

pub fn dot_product(a: Tuple, b: Tuple) -> f32 {
	a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
}

pub fn cross_product(a: Tuple, b: Tuple) -> Tuple {
	new_vector(
		a.y * b.z - a.z * b.y,
		a.z * b.x - a.x * b.z,
		a.x * b.y - a.y * b.x,
	)
}
