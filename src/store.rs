use std::collections::HashMap;
use crate::gen::EntityActive;
use super::gen::EntityID;

/// EcsStore trait
///
/// # Examples
///
/// ```
///
/// ```
///
pub trait EcsStore<T> {
	fn add( &mut self, gen: EntityActive, t: T );
	fn get( &self, gen: EntityActive ) -> Option<&T>;
	fn get_mut( &mut self, gen: EntityActive ) -> Option<&mut T>;
	fn drop( &mut self, gen: EntityActive );
	fn for_each<F: FnMut( EntityActive, &T )>( &self, func: F );
	fn for_each_mut<F: FnMut( EntityActive, &mut T )>( &mut self, func: F );
	fn len( &self ) -> usize;
}

/// HashStore Struct
///
/// # Examples
///
/// ```
///
/// ```
///
pub struct HashStore<T> {
	items: HashMap<EntityActive, T>,
}

impl<T> HashStore<T> {
	
	/// Returns a new instance of a HashStore
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	pub fn new( ) -> Self {
		HashStore { items: HashMap::new( ) }
	}
}

impl<T> EcsStore<T> for HashStore<T> {
	
	/// Adds a entity with ID gen to the HashStore
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	fn add(&mut self, id: EntityActive, t: T) {
		self.items.insert( id, t );
	}
	
	/// Returns a reference to data associated with entity ID gen
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	fn get(&self, id: EntityActive ) -> Option<&T> {
		self.items.get( &id )
	}
	
	/// Returns a mutable reference to data associated with entity ID gen
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	fn get_mut(&mut self, id: EntityActive) -> Option<&mut T> {
		self.items.get_mut( &id )
	}
	
	/// Removes entity with ID gen from the HashStore
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	fn drop(&mut self, id: EntityActive) {
		self.items.remove( &id );
	}
	
	/// Applies a mutable function to each entity in the HashStore but can not mutate the entities themselves
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	fn for_each<F: FnMut(EntityActive, &T)>(&self, mut func: F) {
		for ( _n, x ) in self.items.iter( ).enumerate( ) {
			if let Some( (id, data) ) = Some( (x.0, x.1) ) {
				func( EntityActive { id: id.id, active: id.active }, data );
			}
		}
	}
	
	/// Applies a mutable function to each entity in the HashStore and can mutate the entities themselves
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	fn for_each_mut<F: FnMut(EntityActive, &mut T)>(&mut self, mut func: F) {
		for ( _n, x ) in self.items.iter_mut( ).enumerate( ) {
			if let Some( (id, data) ) = Some( (x.0, x.1) ) {
				func( EntityActive { id: id.id, active: id.active }, data );
			}
		}
	}
	
	/// Returns the number of entities in the HashStore
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	fn len( &self ) -> usize {
		self.items.len()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::gen::{ GenManager };
	
	#[test]
	fn test_store_can_drop( ) {
		/*let mut gen_manager = GenManager::new( );
		let mut vec_store = HashStore::new( );
		
		vec_store.add( gen_manager.next( ), 5 );
		vec_store.add( gen_manager.next( ), 3 );
		vec_store.add( gen_manager.next( ), 8 );
		
		let g4 = gen_manager.next( );
		
		vec_store.add( g4, 9 );
		
		vec_store.for_each_mut( | _gen, data| *data += 2);
		
		assert_eq!( vec_store.get( g4 ), Some( &11 ) );
		
		vec_store.drop( g4 );
		
		assert_eq!( vec_store.get( g4 ), None );*/
	}
}