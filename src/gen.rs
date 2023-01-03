use rand::{RngCore, thread_rng};

/// Entity Active Struct
///
/// # Examples
///
/// ```
///
/// ```
///
#[derive( Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct Entity {
	pub(crate) active: bool,
	pub(crate) id: u64,
}

/// Generation Manager Struct
///
/// # Examples
///
/// ```
///
/// ```
///
//Where we get a new GenerationIDs from
#[derive(Debug)]
pub struct EntityManager {
	items: Vec<Entity>,
	drops: Vec<u64>, // List of all dropped entities
}

impl EntityManager {
	
	/// Returns a new Generation Manager
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	pub fn new( ) -> Self {
		EntityManager {
			items: Vec::new( ),
			drops: Vec::new( ),
		}
	}
	
	/// Returns the length of the items vector, which is the number of active entities
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	pub fn len( &self ) -> usize {
		self.items.len()
	}
	
	/// Returns a new or recently freed GenerationID
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	pub fn next( &mut self ) -> Entity {
		if let Some( id ) = self.drops.pop( ) {
			// Most recent drop
			let entity_active = Entity {
				active: true,
				id,
			};
			self.items.push( entity_active.clone() );
			return entity_active;
		}
		// If nothing left in drops, add on the end
		let entity_active = Entity { active: true, id: thread_rng().next_u64() };
		self.items.push( entity_active.clone() );
		return entity_active;
		
	}
	
	/// Adds entity to the drop list, as long as there is not a newer entity with that ID
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	pub fn drop( &mut self, entity_active: &mut Entity ) {
		if entity_active.active {
			entity_active.active = false;
			self.drops.push( entity_active.id.clone() );
		}
	}
}

#[cfg(test)]
mod tests {
	//use super::*;
	
	#[test]
	fn test_items_drop( ) {
		/*let mut gen_manager = EntityManager::new( );
		
		let g = gen_manager.next( );
		//assert_eq!( g, EntityID { gen: 0, pos: 0 } );
		let g2 = gen_manager.next( );
		gen_manager.next( );
		gen_manager.next( );
		gen_manager.drop( g2 );
		let g3 = gen_manager.next( );
		assert_eq!( g3, EntityID { gen:1, pos: 1 } );*/
	}
}