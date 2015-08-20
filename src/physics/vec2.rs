#[derive(Clone,Copy)]
pub struct Vec2<T> {
	pub x : T,
	pub y : T
}

use num::traits::*;

impl<N: Num + Copy> Vec2<N> {
	pub fn length2(&self) -> N {
		self.x * self.x + self.y * self.y
	}
}

impl<N: Num + Float> Vec2<N> {
	pub fn length(&self) -> N {
		self.length2().sqrt()
	}
}

use std::ops::Add;
impl<N: Num + Add> Add for Vec2<N> {
	type Output = Vec2<N>;

	fn add(self, rhs:Self::Output) -> Self::Output {
		Vec2 { x:self.x + rhs.x, y:self.y + rhs.y }
	}
}

use std::ops::Sub;
impl<N: Num + Sub> Sub for Vec2<N> {
	type Output = Vec2<N>;

	fn sub(self, rhs:Self::Output) -> Self::Output {
		Vec2 { x:self.x - rhs.x, y:self.y - rhs.y }
	}
}

use std::ops::Neg;
impl<N: Signed + Neg> Neg for Vec2<N> {
	type Output = Vec2<N>;

	fn neg(self) -> Self::Output {
		Vec2 { x: -self.x, y: -self.y}
	}
}

use std::ops::Mul;
impl<N: Num + Mul> Mul for Vec2<N> {
	type Output = Vec2<N>;

	fn mul(self, rhs:Self::Output) -> Self::Output {
		Vec2 { x:self.x * rhs.x, y:self.y * rhs.y }
	}
}

impl<N: Num + Mul + Copy> Mul<N> for Vec2<N> {
	type Output = Vec2<N>;

	fn mul(self, rhs : N) -> Self::Output {
		Vec2 { x: self.x * rhs, y: self.y * rhs }
	}
}

use std::ops::Div;
impl<N: Num + Div + Copy> Div<N> for Vec2<N> {
	type Output = Vec2<N>;

	fn div(self, rhs:N) -> Self::Output {
		Vec2 { x:self.x / rhs, y:self.y / rhs}
	}
}

use std::cmp::Ordering;
use std::cmp::PartialEq;
impl<N: Num + PartialEq> PartialEq for Vec2<N> {
	fn eq(&self, rhs:&Vec2<N>) -> bool {
		self.x == rhs.x && self.y == rhs.y
	}
}

use std::cmp::PartialOrd;
impl<N: Num + PartialOrd> PartialOrd for Vec2<N> {
	fn partial_cmp(&self, rhs:&Vec2<N>) -> Option<Ordering> {
		if self.x > rhs.x && self.y > rhs.y {
			return Some(Ordering::Greater)
		} else if self.x < rhs.x && self.y < rhs.y {
			return Some(Ordering::Less)
		}
		None
	}
}
