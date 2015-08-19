extern crate num;
extern crate rand;
extern crate graphics;
extern crate piston_window;
extern crate opengl_graphics;

use num::traits::*;
use piston_window::*;

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
	mass : T
}

fn main() {
	let window : PistonWindow = WindowSettings::new("space", [800, 600]).exit_on_esc(true).build().unwrap();
	
	let mut planets = gen_planets(10);

	for e in window {
		e.draw_2d(|context, device| {
			clear([0.0, 0.0, 0.0, 1.0], device);
			for planet in &mut planets {
				planet.integrate(0.01);
				ellipse(
					[1.0, 0.0, 0.0, 1.0], // red
                    [
                    	planet.position.x*(e.size().width as f64),
                    	planet.position.y*(e.size().height as f64),
                    	10.0, 10.0
                    ],
                    context.transform, device
                );
			}
		});
	}
    println!("Hello, world!");
}

fn gen_planets(num_planets : u32) -> Vec<Entity<f64>> {
	let mut ret : Vec<Entity<f64>> = vec![];
	for _ in 0..num_planets {
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
					x:rand::random::<f64>(),
					y:rand::random::<f64>()
				},
				mass:100.0
			}
		)
	}
	return ret;
}

impl Entity<f64> {
	fn integrate(&mut self, deltatime : f64) -> &mut Entity<f64> {
		self.position = self.position + self.velocity * deltatime;
		self.velocity = self.velocity + self.acceleration * deltatime;
		return self;
	}
}

use std::ops::Add;
impl<N : Num> Add for Vec2<N> {
	type Output = Vec2<N>;

	fn add(self, rhs:Self::Output) -> Self::Output {
		return Vec2 { x:self.x + rhs.x, y:self.y + rhs.y };
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

