use physics::vec2::Vec2;


#[derive(Clone,Copy)]
pub struct Entity<T> {
	pub position : Vec2<T>,
	pub velocity : Vec2<T>,
	pub acceleration : Vec2<T>,
	pub force : Vec2<T>,
	pub mass : T,
	pub radius : T
}

impl Entity<f64> {
	#[inline]
	pub fn get_momentum(&self) -> Vec2<f64> { self.velocity * self.mass }

	#[inline]
	pub fn set_acceleration(&mut self, acceleration : Vec2<f64>) -> &mut Entity<f64> {
		self.acceleration = acceleration;
		self
	}

	#[inline]
	pub fn apply_force(&mut self, force : Vec2<f64>) -> &mut Entity<f64> {
		self.force = self.force + force;
		self
	}

	#[inline]
	pub fn set_force(&mut self, force : Vec2<f64>) -> &mut Entity<f64> {
		self.force = force;
		self
	}

	#[inline]
	pub fn integrate_position(&mut self, deltatime : f64) -> &mut Entity<f64> {
		self.position = self.position + self.velocity * deltatime;
		self
	}

	#[inline]
	pub fn integrate_velocity(&mut self, deltatime : f64) -> &mut Entity<f64> {
		self.velocity = self.velocity + (self.acceleration+(self.force/self.mass)) * deltatime;
		self
	}

	#[inline]
	pub fn get_state_after_collision(mut self, other : Entity<f64>) -> Entity<f64> {
		let numerator = self.velocity * (self.mass - other.mass) + (other.velocity * 2.0 * other.mass);
		let denominator = self.mass + other.mass;
		self.velocity = numerator / denominator;
		self
	}

	#[inline]
	pub fn collides(&self, other : &Entity<f64>) -> bool {
		(self.position - other.position).length2() < (self.radius + other.radius).powi(2)
	}


	#[inline]
	pub fn integrate(&mut self, deltatime : f64) -> &mut Entity<f64> {
		self.integrate_position(deltatime)
		    .integrate_velocity(deltatime)
		    .set_acceleration(Vec2{x:0.0, y:0.0})
		    .set_force(Vec2{x:0.0, y:0.0})
	}

	#[inline]
	pub fn handle_collision(&mut self, other : &mut Entity<f64>) -> &mut Entity<f64> {
		if self.collides(other) {
			let new_self = self.get_state_after_collision(*other);
			let new_other = other.get_state_after_collision(*self);
			*self = new_self;
			*other = new_other;
		}
		self
	}

	#[inline]
	pub fn handle_collisions(&mut self, others : &mut [Entity<f64>]) -> &mut Entity<f64> {
		for other in others {
			self.handle_collision(other);
		}
		self
	}

	#[inline]
	pub fn apply_gravity(&mut self, other : &mut Entity<f64>) -> &mut Entity<f64> {
		const G : f64 = 6.67384 * 1e-11;
		let delta = self.position - other.position;
		let r2 = (delta).length2();
		let dir = delta / r2.sqrt();
		let f = G * (self.mass * other.mass) / r2;
		self.apply_force(dir * -f);
		other.apply_force(dir * f);
		self
	}

	#[inline]
	pub fn apply_gravity_multi(&mut self, others : &mut [Entity<f64>]) -> &mut Entity<f64> {
		for other in others {
			self.apply_gravity(other);
		}
		self
	}

	pub fn handle_wall_collisions(&mut self) -> &mut Entity<f64> {
		if self.position.x - self.radius < 0.0 {
			self.velocity.x = -self.velocity.x;
			self.position.x = 0.0+self.radius;
		}
		else if self.position.x + self.radius > 1.0 {
			self.velocity.x = -self.velocity.x;
			self.position.x = 1.0-self.radius;
		}
		if self.position.y - self.radius < 0.0 {
			self.velocity.y = -self.velocity.y;
			self.position.y = 0.0+self.radius;
		}
		else if  self.position.y + self.radius > 1.0 {
			self.velocity.y = -self.velocity.y;
			self.position.y = 1.0-self.radius;
		}
		self
	}
}
