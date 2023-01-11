use std::collections::hash_map::{IntoIter, Iter, IterMut};
use std::collections::HashMap;
use crate::prelude::Entity;

/// Store trait
///
/// # Examples
///
/// ```
///
/// ```
///
pub trait Store<T> {
	fn add( &mut self, entity: Entity, t: T );
	fn get( &self, entity: Entity ) -> Option<&T>;
	fn get_mut( &mut self, entity: Entity ) -> Option<&mut T>;
	fn drop( &mut self, entity: Entity );
	fn for_each<F: FnMut( Entity, &T )>( &self, func: F );
	fn for_each_mut<F: FnMut( Entity, &mut T )>( &mut self, func: F );
	fn iter( &self ) -> Iter< '_, Entity, T >;
	fn iter_mut( &mut self ) -> IterMut< '_, Entity, T >;
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
	items: HashMap<Entity, T>,
}

impl<T> IntoIterator for HashStore<T> {
	type Item = (Entity, T);
	type IntoIter = IntoIter<Entity, T>;
	
	fn into_iter(self) -> Self::IntoIter {
		self.items.into_iter()
	}
}

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
	fn add(&mut self, id: Entity, t: T) {
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
	fn get(&self, id: Entity ) -> Option<&T> {
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
	fn get_mut(&mut self, id: Entity) -> Option<&mut T> {
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
	fn drop(&mut self, id: Entity) {
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
	fn for_each<F: FnMut(Entity, &T)>(&self, mut func: F) {
		for ( _n, x ) in self.items.iter( ).enumerate( ) {
			if let Some( ( entity, data) ) = Some( (x.0, x.1) ) {
				func( Entity { id: entity.id, active: entity.active }, data );
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
	fn for_each_mut<F: FnMut(Entity, &mut T)>(&mut self, mut func: F) {
		for ( _n, x ) in self.items.iter_mut( ).enumerate( ) {
			if let Some( ( entity, data) ) = Some( (x.0, x.1) ) {
				func( Entity { id: entity.id, active: entity.active }, data );
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
	fn iter( &self ) -> Iter< '_, Entity, T > {
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
	fn iter_mut( &mut self ) -> IterMut< '_, Entity, T > {
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