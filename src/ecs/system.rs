use rand::{RngCore, thread_rng};

///
///
/// # Examples
///
/// ```
///
/// ```
///
pub trait System {
	type SystemData;
	
	fn run( &mut self, data: Self::SystemData );
}

///
///
/// # Examples
///
/// ```
///
/// ```
///
#[derive( Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct SystemID {
	id: u64,
}

impl SystemID {
	
	pub fn new() -> Self {
		SystemID {
			id: thread_rng().next_u64(),
		}
	}
}