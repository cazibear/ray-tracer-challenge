use crate::tuple::{
	Tuple, cross_product, dot_product, float_equal, magnitude, new_point, new_vector, normalize,
};

#[path = "../src/tuple.rs"]
mod tuple;

// Tuples

#[test]
fn tuples_1() {
	// Scenario: A tuple with w=1.0 is a point
	let a = Tuple {
		x: 4.3,
		y: -4.2,
		z: 3.1,
		w: 1.0,
	};
	assert_eq!(a.x, 4.3);
	assert_eq!(a.y, -4.2);
	assert_eq!(a.z, 3.1);
	assert_eq!(a.w, 1.0);
	assert!(a.is_point());
	assert!(!a.is_vector());
}

#[test]
fn tuples_2() {
	// Scenario: A tuple with w=0 is a vector
	let b = Tuple {
		x: 4.3,
		y: -4.2,
		z: 3.1,
		w: 0.0,
	};
	assert_eq!(b.x, 4.3);
	assert_eq!(b.y, -4.2);
	assert_eq!(b.z, 3.1);
	assert_eq!(b.w, 0.0);
	assert!(!b.is_point());
	assert!(b.is_vector());
}

#[test]
fn tuples_3() {
	// Scenario: point() creates tuples with w=1
	let p = new_point(4.0, -4.0, 3.0);
	assert_eq!(
		p,
		Tuple {
			x: 4.0,
			y: -4.0,
			z: 3.0,
			w: 1.0
		}
	);
}

#[test]
fn tuples_4() {
	// Scenario: vector() creates tuples with w=0
	let v = new_vector(4.0, -4.0, 3.0);
	assert_eq!(
		v,
		Tuple {
			x: 4.0,
			y: -4.0,
			z: 3.0,
			w: 0.0
		}
	);
}

// Operations

#[test]
fn operations_1() {
	// Scenario: Adding two tuples
	let a1 = Tuple {
		x: 3.0,
		y: -2.0,
		z: 5.0,
		w: 1.0,
	};
	let a2 = Tuple {
		x: -2.0,
		y: 3.0,
		z: 1.0,
		w: 0.0,
	};
	assert_eq!(
		a1 + a2,
		Tuple {
			x: 1.0,
			y: 1.0,
			z: 6.0,
			w: 1.0
		}
	);
}

#[test]
fn operations_2() {
	// Scenario: Subtracting two points
	let p1 = new_point(3.0, 2.0, 1.0);
	let p2 = new_point(5.0, 6.0, 7.0);
	assert_eq!(p1 - p2, new_vector(-2.0, -4.0, -6.0));
}

#[test]
fn operations_3() {
	// Scenario: Subtracting a vector from a point
	let p = new_point(3.0, 2.0, 1.0);
	let v = new_vector(5.0, 6.0, 7.0);
	assert_eq!(p - v, new_point(-2.0, -4.0, -6.0));
}

#[test]
fn operations_4() {
	// Scenario: Subtracting two vectors
	let v1 = new_vector(3.0, 2.0, 1.0);
	let v2 = new_vector(5.0, 6.0, 7.0);
	assert_eq!(v1 - v2, new_vector(-2.0, -4.0, -6.0));
}

#[test]
fn operations_5() {
	// Scenario: Subtracting a vector from the zero vector
	let zero = new_vector(0.0, 0.0, 0.0);
	let v = new_vector(1.0, -2.0, 3.0);
	assert_eq!(zero - v, new_vector(-1.0, 2.0, -3.0));
}

#[test]
fn operations_6() {
	// Scenario: Negating a tuple
	let a = Tuple {
		x: 1.0,
		y: -2.0,
		z: 3.0,
		w: -4.0,
	};
	assert_eq!(
		-a,
		Tuple {
			x: -1.0,
			y: 2.0,
			z: -3.0,
			w: 4.0
		}
	);
}

// Scalar Multiplication And Division

#[test]
fn operations_7() {
	// Scenario: Multiplying a tuple by a scalar
	let a = Tuple {
		x: 1.0,
		y: -2.0,
		z: 3.0,
		w: -4.0,
	};
	assert_eq!(
		a * 3.5,
		Tuple {
			x: 3.5,
			y: -7.0,
			z: 10.5,
			w: -14.0
		}
	)
}

#[test]
fn operations_8() {
	// Scenario: Multiplying a tuple by a fraction
	let a = Tuple {
		x: 1.0,
		y: -2.0,
		z: 3.0,
		w: -4.0,
	};
	assert_eq!(
		a * 0.5,
		Tuple {
			x: 0.5,
			y: -1.0,
			z: 1.5,
			w: -2.0
		}
	)
}

#[test]
fn operations_9() {
	// Scenario: Dividing a tuple by a scalar
	let a = Tuple {
		x: 1.0,
		y: -2.0,
		z: 3.0,
		w: -4.0,
	};
	assert_eq!(
		a / 2,
		Tuple {
			x: 0.5,
			y: -1.0,
			z: 1.5,
			w: -2.0
		}
	);
}

// Magnitude

#[test]
fn operations_10() {
	// Scenario: Computing the magnitude of vector(1, 0, 0)
	let v = new_vector(1.0, 0.0, 0.0);
	assert!(float_equal(magnitude(v), 1.0));
}

#[test]
fn operations_11() {
	// Scenario: Computing the magnitude of vector(0, 1, 0)
	let v = new_vector(0.0, 1.0, 0.0);
	assert!(float_equal(magnitude(v), 1.0));
}

#[test]
fn operations_12() {
	// Scenario: Computing the magnitude of vector(0, 0, 1)
	let v = new_vector(0.0, 0.0, 1.0);
	assert!(float_equal(magnitude(v), 1.0));
}

#[test]
fn operations_13() {
	// Scenario: Computing the magnitude of vector(1, 2, 3)
	let v = new_vector(1.0, 2.0, 3.0);
	assert!(float_equal(magnitude(v), 14_f32.sqrt()));
}

#[test]
fn operations_14() {
	// Scenario: Computing the magnitude of vector(-1, -2, -3)
	let v = new_vector(-1.0, -2.0, -3.0);
	assert!(float_equal(magnitude(v), 14_f32.sqrt()));
}

// Normalization

#[test]
fn operations_15() {
	// Scenario: Normalizing vector(4, 0, 0) gives (1, 0, 0)
	let v = new_vector(4.0, 0.0, 0.0);
	assert_eq!(normalize(v), new_vector(1.0, 0.0, 0.0));
}

#[test]
fn operations_16() {
	// Scenario: Normalizing vector(1, 2, 3)
	let v = new_vector(1.0, 2.0, 3.0);
	assert_eq!(
		normalize(v),
		new_vector(
			1.0 / 14_f32.sqrt(),
			2.0 / 14_f32.sqrt(),
			3.0 / 14_f32.sqrt()
		)
	);
	assert_eq!(normalize(v), new_vector(0.26726, 0.53452, 0.80178));
}

#[test]
fn operations_17() {
	// Scenario: The magnitude of a normalized vector
	let v = new_vector(1.0, 2.0, 3.0);
	assert!(float_equal(magnitude(normalize(v)), 1.0));
}

// Products

#[test]
fn operations_18() {
	// Scenario: The dot product of two tuples
	let a = new_vector(1.0, 2.0, 3.0);
	let b = new_vector(2.0, 3.0, 4.0);
	assert!(float_equal(dot_product(a, b), 20.0));
}

#[test]
fn operations_19() {
	// Scenario: The cross product of two vectors
	let a = new_vector(1.0, 2.0, 3.0);
	let b = new_vector(2.0, 3.0, 4.0);
	assert_eq!(cross_product(a, b), new_vector(-1.0, 2.0, -1.0));
	assert_eq!(cross_product(b, a), new_vector(1.0, -2.0, 1.0));
}
