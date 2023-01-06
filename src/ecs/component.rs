use rand::{RngCore, thread_rng};

///
///
/// # Examples
///
/// ```
///
/// ```
///
pub trait Component {
	type Storage;
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
pub struct ComponentID {
	id: u64,
}

impl ComponentID {
	
	pub fn new() -> Self {
		ComponentID {
			id: thread_rng().next_u64(),
		}
	}
}