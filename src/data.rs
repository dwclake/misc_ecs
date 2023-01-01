use raylib::prelude::Color;

///
// todo: add things like mass which can be used in the collision system to determine how velocities should be changed
pub struct Attributes {
	pub mass: f64,
	pub color: Color,
	pub radius: f64,
}