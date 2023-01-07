use std::collections::HashMap;
use rand::{RngCore, thread_rng};
use crate::prelude::*;

/// Entity Active Struct
///
/// # Examples
///
/// ```
///
/// ```
///
#[derive( Eq, Hash, Copy, Clone, Debug)]
pub struct EntityID {
	pub(crate) active: bool,
	pub(crate) id: u64,
}

impl PartialEq for EntityID {
	fn eq(&self, rhs: &Self) -> bool {
		if self.active == rhs.active && self.id == rhs.id {
			true
		} else {
			false
		}
	}
}

impl EntityID {
	
	///
	pub fn is_active(&self) -> bool {
		self.active
	}
	
	///
	pub fn id(&self) -> u64 {
		self.id
	}
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
	items: HashMap<u64, EntityID>,
	drops: Vec<EntityID>, // List of all dropped entities
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
			items: HashMap::new( ),
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
		if let Some( entity ) = self.drops.pop( ) {
			// Most recent drop
			let entity = EntityID {
				active: true,
				id: entity.id,
			};
			self.items.insert( entity.id, entity.clone() );
			return entity;
		}
		// If nothing left in drops, add on the end
		let entity = EntityID { active: true, id: thread_rng().next_u64() };
		self.items.insert( entity.id, entity.clone() );
		return entity;
		
	}
	
	/// Adds entity to the drop list, as long as there is not a newer entity with that ID
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	pub fn drop( &mut self, entity: &mut EntityID ) {
		if entity.active {
			entity.active = false;
			self.drops.push( *entity );
			self.items.remove( &entity.id );
		}
	}
}


pub trait Builder<T> {
	
	///
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	fn with<C: Component>(self, component: C) -> Self;
	
	///
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	fn build(self) -> EntityID;
}

///
///
/// # Examples
///
/// ```
///
/// ```
///
pub struct EntityBuilder<'a> {
	pub(crate) entity: EntityID,
	pub(crate) _world: &'a World,
	pub(crate) built: bool,
}

impl<'a, T> Builder<T> for EntityBuilder<'a> {
	
	fn with<C: Component>(self, _component: C) -> Self {
		{
			//let mut storage: C = SystemData::fetch(self.world);
			//storage.insert(self.entity, c).unwrap();
		}
		
		self
	}
	
	fn build(mut self) -> EntityID {
		self.built = true;
		self.entity
	}
}

/*#[cfg(test)]
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
}*/