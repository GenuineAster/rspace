#![feature(test)]

extern crate num;
extern crate test;
extern crate rand;
extern crate graphics;
extern crate piston_window;
extern crate opengl_graphics;

use rand::distributions::{IndependentSample, Range};

mod physics;
use physics::entity::*;
use physics::vec2::*;

#[cfg(not(test))]
use piston_window::*;

#[cfg(not(test))]
fn main() {
	let window_settings = WindowSettings::new("space", [600, 600])
	    .opengl(opengl_graphics::OpenGL::V2_1);
	let (ma, mi) = window_settings.get_maybe_opengl().unwrap().get_major_minor();
	let window : PistonWindow = window_settings
		.exit_on_esc(true)
		.build()
		.unwrap();

	println!("OpenGL version: {}.{}", ma, mi);

	
	let mut planets = gen_planets(50);
	let step_time = 3.0;

	for e in window {
		e.draw_2d(|context, device| {
			clear([0.0, 0.0, 0.0, 1.0], device);
			for i in 0..planets.len() {
				{
					let (planets_i, planets_j) = planets.split_at_mut(i+1);
					planets_i[i].integrate(step_time)
					            .handle_wall_collisions()
					            .handle_collisions(planets_j)
					            .apply_gravity_multi(planets_j);
				}

				{
					let total_momentum : f64 = planets.iter()
						.map(
							|&planet| planet.get_momentum().length()
						).fold(0.0,|acc, momentum| acc + momentum);

					println!("Total momentum in system: {}", total_momentum);
				}

				{
					let base = 0.06;

					let m = planets[i].get_momentum().length() as f32;
					let color = [m*10.0 + base, m*5.0 + base, m*2.0 + 0.15 + base, 1.0];
					let e_size = Vec2 {x: e.size().width as f64, y: e.size().height as f64};

					let position = planets[i].position * e_size - e_size*planets[i].radius;
					let size     = e_size * planets[i].radius * 2.0;

					ellipse(
						color,
						[
							position.x, position.y,
							size.x, size.y
						],
						context.transform, device
					);
				}
			}
		});
	}
}

use num::traits::*;
fn gen_planets(num_planets : u32) -> Vec<Entity<f64>> {
	let mut ret : Vec<Entity<f64>> = vec![];
	let mut rng = rand::thread_rng();
	let rad_range = Range::new(1.0/1000.0, 10.0/1000.0);
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
					x:0.0,
					y:0.0
				},
				force: Vec2 {
					x:0.0,
					y:0.0
				},
				mass:(rad_mass*1000.0).powi(2),
				radius:rad_mass
			}
		)
	}
	ret
}


#[cfg(test)]
mod tests {
	use super::gen_planets;
	use test::Bencher;


	static BENCH_PLANETS : u32 = 1000;

	#[bench]
	fn bench_planet_collision_func(b: &mut Bencher) {
		let mut planets = gen_planets(BENCH_PLANETS);
		b.iter(||
			for i in 0..planets.len() {
				let (planets_i, planets_j) = planets.split_at_mut(i+1);
				planets_i[i].integrate(0.01)
				            .handle_wall_collisions()
				            .handle_collisions(planets_j)
				            .apply_gravity_multi(planets_j);
			}
		)
	}
}
