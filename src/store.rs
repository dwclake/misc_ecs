use super::gen::GenData;

// This could be implemented by Vec type object, or tree or hashmap
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

///
pub struct VecStore<T> {
	items: Vec<Option<( u64, T )>>,
}

impl<T> VecStore<T> {
	
	///
	pub fn new( ) -> Self {
		VecStore { items: Vec::new( ) }
	}
}

impl<T> EcsStore<T> for VecStore<T> {
	
	///
	fn add(&mut self, gen: GenData, t: T) {
		while gen.pos >= self.items.len( ) {
			self.items.push( None );
		}
		self.items[gen.pos] = Some( ( gen.gen, t ) );
	}
	
	///
	fn get(&self, gen: GenData) -> Option<&T> {
		if let Some( Some( ( in_gen, data ) ) ) = self.items.get( gen.pos ) {
			if *in_gen == gen.gen {
				return Some( data );
			}
		}
		None
	}
	
	///
	fn get_mut(&mut self, gen: GenData) -> Option<&mut T> {
		if let Some( Some( ( in_gen, data ) ) ) = self.items.get_mut( gen.pos ) {
			if *in_gen == gen.gen {
				return Some( data );
			}
		}
		None
	}
	
	///
	fn drop(&mut self, gen: GenData) {
		if let Some( Some( ( in_gen, _  ) ) ) = self.items.get( gen.pos ) {
			if *in_gen == gen.gen {
				self.items[gen.pos] = None;
			}
		}
	}
	
	///
	fn for_each<F: FnMut(GenData, &T)>(&self, mut func: F) {
		for ( n, x ) in self.items.iter( ).enumerate( ) {
			if let Some( ( gen, data ) ) = x {
				func( GenData { gen: *gen, pos: n }, data );
			}
		}
	}
	
	///
	fn for_each_mut<F: FnMut(GenData, &mut T)>(&mut self, mut func: F) {
		for ( n, x ) in self.items.iter_mut( ).enumerate( ) {
			if let Some( ( gen, data ) ) = x {
				func( GenData { gen: *gen, pos: n }, data );
			}
		}
	}
	
	///
	fn len( &self ) -> usize {
		self.items.len()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ecs::gen::{ GenManager };
	
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