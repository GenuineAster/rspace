#[derive(Clone,Copy)]
pub struct Vec2<T> {
	pub x : T,
	pub y : T
}

impl Vec2<f64> {
	pub fn length2(&self) -> f64 {
		self.x * self.x + self.y * self.y
	}

	pub fn length(&self) -> f64 {
		self.length2().sqrt()
	}
}

use num::traits::*;

use std::ops::Add;
impl<N : Num> Add for Vec2<N> {
	type Output = Vec2<N>;

	fn add(self, rhs:Self::Output) -> Self::Output {
		return Vec2 { x:self.x + rhs.x, y:self.y + rhs.y };
	}
}

use std::ops::Sub;
impl<N : Num> Sub for Vec2<N> {
	type Output = Vec2<N>;

	fn sub(self, rhs:Self::Output) -> Self::Output {
		return Vec2 { x:self.x - rhs.x, y:self.y - rhs.y };
	}
}

use std::ops::Neg;
impl Neg for Vec2<f64> {
	type Output = Vec2<f64>;

	fn neg(self) -> Self::Output {
		return Vec2 { x: -self.x, y: -self.y};
	}
}

use std::ops::Mul;
impl<N : Num> Mul for Vec2<N> {
	type Output = Vec2<N>;

	fn mul(self, rhs:Self::Output) -> Self::Output {
		Vec2 { x:self.x * rhs.x, y:self.y * rhs.y }
	}
}

impl Mul<f64> for Vec2<f64> {
	type Output = Vec2<f64>;

	fn mul(self, rhs:f64) -> Self::Output {
		return Vec2 { x:self.x * rhs, y:self.y * rhs};
	}
}

use std::ops::Div;
impl Div<f64> for Vec2<f64> {
	type Output = Vec2<f64>;

	fn div(self, rhs:f64) -> Self::Output {
		return Vec2 { x:self.x / rhs, y:self.y / rhs};
	}
}

use std::cmp::Ordering;
use std::cmp::PartialEq;
impl<N : Num> PartialEq for Vec2<N> {
	fn eq(&self, rhs:&Vec2<N>) -> bool {
		self.x == rhs.x && self.y == rhs.y
	}
}

use std::cmp::PartialOrd;
impl PartialOrd for Vec2<f64> {
	fn partial_cmp(&self, rhs:&Vec2<f64>) -> Option<Ordering> {
		if self.x > rhs.x && self.y > rhs.y {
			return Some(Ordering::Greater);
		} else if self.x < rhs.x && self.y < rhs.y {
			return Some(Ordering::Less);
		}
		return None;
	}
}
