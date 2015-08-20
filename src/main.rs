extern crate num;
extern crate rand;
extern crate graphics;
extern crate piston_window;
extern crate opengl_graphics;

use num::traits::*;
use piston_window::*;
use rand::distributions::{IndependentSample, Range};

#[derive(Clone,Copy)]
struct Vec2<T> {
	x : T,
	y : T
}

#[derive(Clone,Copy)]
struct Entity<T> {
	position : Vec2<T>,
	velocity : Vec2<T>,
	acceleration : Vec2<T>,
	mass : T,
	radius : T
}

fn main() {
	let window : PistonWindow = WindowSettings::new("space", [600, 600]).exit_on_esc(true).build().unwrap();
	
	let mut planets = gen_planets(10);

	for e in window {
		e.draw_2d(|context, device| {
			clear([0.0, 0.0, 0.0, 1.0], device);
			for i in 0..planets.len() {
				planets[i].integrate(0.01).handle_wall_collision().integrate(0.01);

				for j in i+1..planets.len() {
					let (planets_i, planets_j) = planets.split_at_mut(i+1);
					planets_i[i].handle_collision(&mut planets_j[j-i-1]);
				}

				ellipse(
					[1.0, 0.0, 0.0, 1.0],
                    [
                    	planets[i].position.x*(e.size().width as f64),
                    	planets[i].position.y*(e.size().height as f64),
                    	planets[i].radius*(e.size().width as f64),
                    	planets[i].radius*(e.size().height as f64)
                    ],
                    context.transform, device
                );
			}
		});
	}
}

fn gen_planets(num_planets : u32) -> Vec<Entity<f64>> {
	let mut ret : Vec<Entity<f64>> = vec![];
	let mut rng = rand::thread_rng();
	let acc_range = Range::new(-1.0, 1.0);
	let rad_range = Range::new(5.0/1000.0, 25.0/1000.0);
	for _ in 0..num_planets {
		let rad_mass = rad_range.ind_sample(&mut rng);
		ret.push(
			Entity {
				position: Vec2 {
					x:rand::random::<f64>(),
					y:rand::random::<f64>()
				},
				velocity: Vec2 {
					x:0.0,
					y:0.0
				},
				acceleration: Vec2 {
					x:acc_range.ind_sample(&mut rng),
					y:acc_range.ind_sample(&mut rng)
				},
				mass:rad_mass,
				radius:rad_mass
			}
		)
	}
	return ret;
}

impl Entity<f64> {
	#[inline(always)]
	fn integrate(&mut self, deltatime : f64) -> &mut Entity<f64> {
		self.position = self.position + self.velocity * deltatime;
		self.velocity = self.velocity + self.acceleration * deltatime;
		self
	}

	#[inline(always)]
	fn handle_collision(&mut self, other : &mut Entity<f64>) -> &mut Entity<f64> {
		let dist_vec = self.position - other.position;
		if dist_vec.length2() < (self.radius + other.radius).powi(2) {
			self.velocity = -self.velocity;
			other.velocity = -other.velocity;
		}
		self
	}

	#[inline(always)]
	fn handle_wall_collision(&mut self) -> &mut Entity<f64> {
		if self.position.x - self.radius < 0.0 {
			self.velocity.x = -self.velocity.x;
			self.position.x = 0.0+self.radius;
		} else if self.position.x + self.radius > 1.0 {
			self.velocity.x = -self.velocity.x;
			self.position.x = 1.0-self.radius;
		}
		if self.position.y - self.radius < 0.0 {
			self.velocity.y = -self.velocity.y;
			self.position.y = 0.0+self.radius;
		} else if  self.position.y + self.radius > 1.0 {
			self.velocity.y = -self.velocity.y;
			self.position.y = 1.0-self.radius;
		}
		self
	}
}

impl Vec2<f64> {
	fn length2(&self) -> f64 {
		self.x * self.x + self.y * self.y
	}
}

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
