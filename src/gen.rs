/// Generation Data Struct
///
/// # Examples
///
/// ```
///
/// ```
///
#[derive( Copy, Clone, Debug, PartialEq )]
pub struct GenData {
	pub(crate) pos: usize,
	pub(crate) gen: u64,
}

/// Entity Active Struct
///
/// # Examples
///
/// ```
///
/// ```
///
#[derive(Debug)]
pub struct EntityActive {
	active: bool,
	gen: u64,
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
	drops: Vec<usize>, // List of all dropped entities
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
	pub fn next( &mut self ) -> GenData {
		if let Some( loc ) = self.drops.pop( ) {
			// Most recent drop
			let entity_active = &mut self.items[loc];
			entity_active.active = true;
			entity_active.gen += 1;
			
			return GenData {
				pos: loc,
				gen: entity_active.gen,
			};
		}
		// If nothing left in drops, add on the end
		self.items.push( EntityActive {active: true,gen: 0});
		GenData {
			gen: 0,
			pos: self.items.len() - 1,
		}
	}
	
	/// Adds entity to the drop list, as long as there is not a newer entity with that ID
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	pub fn drop( &mut self, gen: GenData ) {
		if let Some( entity_active ) = self.items.get_mut( gen.pos ) {
			if entity_active.active && entity_active.gen == gen.gen {
				// Don't drop newer items than given
				entity_active.active = false;
				self.drops.push( gen.pos );
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_items_drop( ) {
		let mut gen_manager = GenManager::new( );
		
		let g = gen_manager.next( );
		assert_eq!( g, GenData { gen: 0, pos: 0 } );
		let g2 = gen_manager.next( );
		gen_manager.next( );
		gen_manager.next( );
		gen_manager.drop( g2 );
		let g3 = gen_manager.next( );
		assert_eq!( g3, GenData { gen:1, pos: 1 } );
	}
}