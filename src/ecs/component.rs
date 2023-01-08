use rand::{RngCore, thread_rng};
use crate::ecs::store::Storage;

///
///
/// # Examples
///
/// ```
///
/// ```
///
pub trait Component {
	type Storage: Storage;
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