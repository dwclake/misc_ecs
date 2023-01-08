use std::collections::hash_map::{IntoIter, Iter, IterMut};
use std::collections::HashMap;
use crate::prelude::EntityID;

/// Store trait
///
/// # Examples
///
/// ```
///
/// ```
///
pub trait Store<T> {
	fn add( &mut self, entity: EntityID, t: T );
	fn get( &self, entity: EntityID ) -> Option<&T>;
	fn get_mut( &mut self, entity: EntityID ) -> Option<&mut T>;
	fn drop( &mut self, entity: EntityID );
	fn for_each<F: FnMut( EntityID, &T )>( &self, func: F );
	fn for_each_mut<F: FnMut( EntityID, &mut T )>( &mut self, func: F );
	fn iter( &self ) -> Iter< '_, EntityID, T >;
	fn iter_mut( &mut self ) -> IterMut< '_, EntityID, T >;
	fn len( &self ) -> usize;
}

/// Store trait
///
/// # Examples
///
/// ```
///
/// ```
///
pub trait Storage {
	fn new() -> Self;
}

/// HashStore Struct
///
/// # Examples
///
/// ```
///
/// ```
///
#[derive(Debug)]
pub struct HashStore<T> {
	items: HashMap<EntityID, T>,
}

impl<T> IntoIterator for HashStore<T> {
	type Item = (EntityID, T);
	type IntoIter = IntoIter<EntityID, T>;
	
	fn into_iter(self) -> Self::IntoIter {
		self.items.into_iter()
	}
}

/*impl<T> Iterator for HashStore<T> {
	type Item = ();
	
	fn next(&mut self) -> Option<Self::Item> {
		todo!()
	}
}*/

impl<T> Storage for HashStore<T> {
	/// Creates a new HashStore
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	fn new( ) -> Self {
		HashStore { items: HashMap::new( ) }
	}
}

impl<T> HashStore<T> {

	
}

impl<T> Store<T> for HashStore<T> {
	
	/// Adds a entity with ID gen to the HashStore
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	fn add(&mut self, id: EntityID, t: T) {
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
	fn get(&self, id: EntityID ) -> Option<&T> {
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
	fn get_mut(&mut self, id: EntityID) -> Option<&mut T> {
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
	fn drop(&mut self, id: EntityID) {
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
	fn for_each<F: FnMut(EntityID, &T)>(&self, mut func: F) {
		for ( _n, x ) in self.items.iter( ).enumerate( ) {
			if let Some( ( entity, data) ) = Some( (x.0, x.1) ) {
				func( EntityID { id: entity.id, active: entity.active }, data );
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
	fn for_each_mut<F: FnMut(EntityID, &mut T)>(&mut self, mut func: F) {
		for ( _n, x ) in self.items.iter_mut( ).enumerate( ) {
			if let Some( ( entity, data) ) = Some( (x.0, x.1) ) {
				func( EntityID { id: entity.id, active: entity.active }, data );
			}
		}
	}
	
	///
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	fn iter( &self ) -> Iter< '_, EntityID, T > {
		self.items.iter()
	}
	
	///
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	fn iter_mut( &mut self ) -> IterMut< '_, EntityID, T > {
		self.items.iter_mut()
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

/*#[cfg(test)]
mod tests {
	//use super::*;
	//use crate::gen::{ EntityIDManager };
	
	#[test]
	fn test_store_can_drop( ) {
		/*let mut gen_manager = EntityIDManager::new( );
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
}*/