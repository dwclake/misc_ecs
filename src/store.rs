use super::gen::GenData;

/// EcsStore trait
///
/// # Examples
///
/// ```
///
/// ```
///
pub trait EcsStore<T> {
	fn add( &mut self, gen: GenData, t: T );
	fn get( &self, gen: GenData ) -> Option<&T>;
	fn get_mut( &mut self, gen: GenData ) -> Option<&mut T>;
	fn drop( &mut self, gen: GenData );
	// Optional but helpful could be another trait even
	fn for_each<F: FnMut( GenData, &T )>( &self, func: F );
	fn for_each_mut<F: FnMut( GenData, &mut T )>( &mut self, func: F );
	fn len( &self ) -> usize;
}

/// VecStore Struct
///
/// # Examples
///
/// ```
///
/// ```
///
pub struct VecStore<T> {
	items: Vec<Option<( u64, T )>>,
}

impl<T> VecStore<T> {
	
	/// Returns a new instance of a VecStore
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	pub fn new( ) -> Self {
		VecStore { items: Vec::new( ) }
	}
}

impl<T> EcsStore<T> for VecStore<T> {
	
	/// Adds a entity with ID gen to the VecStore
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	fn add(&mut self, gen: GenData, t: T) {
		while gen.pos >= self.items.len( ) {
			self.items.push( None );
		}
		self.items[gen.pos] = Some( ( gen.gen, t ) );
	}
	
	/// Returns a reference to data associated with entity ID gen
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	fn get(&self, gen: GenData) -> Option<&T> {
		if let Some( Some( ( in_gen, data ) ) ) = self.items.get( gen.pos ) {
			if *in_gen == gen.gen {
				return Some( data );
			}
		}
		None
	}
	
	/// Returns a mutable reference to data associated with entity ID gen
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	fn get_mut(&mut self, gen: GenData) -> Option<&mut T> {
		if let Some( Some( ( in_gen, data ) ) ) = self.items.get_mut( gen.pos ) {
			if *in_gen == gen.gen {
				return Some( data );
			}
		}
		None
	}
	
	/// Removes entity with ID gen from the VecStore
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	fn drop(&mut self, gen: GenData) {
		if let Some( Some( ( in_gen, _  ) ) ) = self.items.get( gen.pos ) {
			if *in_gen == gen.gen {
				self.items[gen.pos] = None;
			}
		}
	}
	
	/// Applies a mutable function to each entity in the VecStore but can not mutate the entities themselves
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	fn for_each<F: FnMut(GenData, &T)>(&self, mut func: F) {
		for ( n, x ) in self.items.iter( ).enumerate( ) {
			if let Some( ( gen, data ) ) = x {
				func( GenData { gen: *gen, pos: n }, data );
			}
		}
	}
	
	/// Applies a mutable function to each entity in the VecStore and can mutate the entities themselves
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	fn for_each_mut<F: FnMut(GenData, &mut T)>(&mut self, mut func: F) {
		for ( n, x ) in self.items.iter_mut( ).enumerate( ) {
			if let Some( ( gen, data ) ) = x {
				func( GenData { gen: *gen, pos: n }, data );
			}
		}
	}
	
	/// Returns the number of entities in the VecStore
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
		let mut gen_manager = GenManager::new( );
		let mut vec_store = VecStore::new( );
		
		vec_store.add( gen_manager.next( ), 5 );
		vec_store.add( gen_manager.next( ), 3 );
		vec_store.add( gen_manager.next( ), 8 );
		
		let g4 = gen_manager.next( );
		
		vec_store.add( g4, 9 );
		
		vec_store.for_each_mut( | _gen, data| *data += 2);
		
		assert_eq!( vec_store.get( g4 ), Some( &11 ) );
		
		vec_store.drop( g4 );
		
		assert_eq!( vec_store.get( g4 ), None );
	}
}