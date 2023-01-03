use rand::{RngCore, thread_rng};

/// Generation Data Struct
///
/// # Examples
///
/// ```
///
/// ```
///
#[derive( Eq, Hash, Copy, Clone, Debug, PartialEq )]
pub struct EntityID {
	pub(crate) id: u64,
}

/// Entity Active Struct
///
/// # Examples
///
/// ```
///
/// ```
///
#[derive( Copy, Clone, Debug)]
pub struct EntityActive {
	active: bool,
	id: EntityID,
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
pub struct GenManager {
	items: Vec<EntityActive>,
	drops: Vec<EntityID>, // List of all dropped entities
}

impl GenManager {
	
	/// Returns a new Generation Manager
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	pub fn new( ) -> Self {
		GenManager {
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
	pub fn next( &mut self ) -> EntityID {
		if let Some( loc ) = self.drops.pop( ) {
			// Most recent drop
			let entity_active = EntityActive {
				active: true,
				id: loc,
			};
			self.items.push( entity_active.clone() );
			return entity_active.id;
		}
		// If nothing left in drops, add on the end
		let entity_active = EntityActive {active: true, id: EntityID{ id: thread_rng().next_u64() }};
		self.items.push( entity_active.clone() );
		return entity_active.id;
		
	}
	
	/// Adds entity to the drop list, as long as there is not a newer entity with that ID
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	pub fn drop( &mut self, entity_active: &mut EntityActive ) {
		//if let Some( entity_active ) = self.items.get_mut(  ) {
		if entity_active.active {
			// Don't drop newer items than given
			entity_active.active = false;
			self.drops.push( entity_active.id.clone() );
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_items_drop( ) {
		/*let mut gen_manager = GenManager::new( );
		
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