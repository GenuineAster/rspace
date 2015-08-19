extern crate num;
extern crate graphics;
extern crate piston_window;
extern crate opengl_graphics;

use piston_window::*;
use num::traits::*;

struct Vec2<T> {
	x : T,
	y : T
}

struct Entity<T> {
	position : Vec2<T>,
	velocity : Vec2<T>,
	acceleration : Vec2<T>,
	mass : T
}

fn main() {
	let window : PistonWindow = WindowSettings::new("space", [800, 600]).exit_on_esc(true).build().unwrap();
	
	let mut planets : Vec<Entity<f64>> = vec![];

	
	planets.push(Entity {
		position: Vec2 {
			x:0.0,
			y:0.0
		},
		velocity: Vec2 {
			x:1.0,
			y:1.0
		},
		acceleration: Vec2 {
			x:1.0,
			y:1.0
		},
		mass:100.0
	});
	

	for e in window {
		e.draw_2d(|context, device| {
			clear([0.0, 0.0, 0.0, 1.0], device);
			for planet in &mut planets {
				integrate(planet, 0.1);
				ellipse([1.0, 0.0, 0.0, 1.0], // red
                      [planet.position.x, planet.position.y, 10.0, 10.0],
                      context.transform, device);
			}
		});
	}
    println!("Hello, world!");
}

fn integrate(entity : &mut Entity<f64>, deltatime : f64) {
	*entity = Entity {
		position: Vec2 {
			x:entity.position.x + entity.velocity.x * deltatime,
			y:entity.position.y + entity.velocity.y * deltatime
		},
		velocity: Vec2 {
			x:entity.velocity.x + entity.acceleration.x * deltatime,
			y:entity.velocity.y + entity.acceleration.y * deltatime
		},
		acceleration: Vec2 {
			x: entity.acceleration.x,
			y: entity.acceleration.y
		},
		mass: entity.mass
	};
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
		return Vec2 { x:self.x + rhs.x, y:self.y + rhs.y };
	}
}

