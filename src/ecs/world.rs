use std::collections::HashMap;
use crate::ecs::component::{ComponentID};
use crate::ecs::entity::EntityBuilder;
use crate::ecs::system::SystemID;
use crate::prelude::{Component, EntityManager, System};

///
///
/// # Examples
///
/// ```
///
/// ```
///
pub struct World {
	entity_manager: EntityManager,
	_components: HashMap< ComponentID, Box<dyn Component<Storage = ()>>>,
	_systems: HashMap< SystemID, Box<dyn System<SystemData = ()>>>,
}

impl World {
	
	///
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	pub fn new() -> World {
		World {
			entity_manager: EntityManager::new(),
			_components: HashMap::new(),
			_systems: HashMap::new(),
		}
	}
	
	///
	pub fn entity_manager(&self) -> &EntityManager {
		&self.entity_manager
	}
	
	///
	pub fn entity_manager_mut(&mut self) -> &mut EntityManager {
		&mut self.entity_manager
	}
	
	///
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	pub fn create_entity( &mut self ) -> EntityBuilder {
		EntityBuilder {
			entity: self.entity_manager.next(),
			_world: self,
			built: false,
		}
	}
	
	/// Register function
	///
	/// # Examples
	///
	/// ```
	/// use std::io::stderr;
	/// use std::ops::Add;
	/// use misc_ecs::ecs::component::ComponentID;
	/// use misc_ecs::ecs::store::Storage;
	/// use misc_ecs::prelude::*;
	///
	/// #[derive(Debug, Copy, Clone)]
	/// struct Pos(f32);
	///
	/// impl Component for Pos {
	/// 	type Storage = HashStore<Self>;
	/// }
	///
	/// let mut world = World::new();
	///
	/// let mut x = HashStore::new();
	/// let ent = world.entity_manager_mut().next();
	/// x.add( ent, Box::new( Pos(42.0) ) );
	///
	/// let acc = ComponentID::new();
	/// world.register::<Pos>( acc );
	///
	/// //assert_eq!( x.len().clone(), 1);
	/// //assert!( &x.get( ent ).unwrap().0 - 42.0 < 0.00001 );
	/// ```
	///
	pub fn register<T: Component >( &mut self, _id: ComponentID ) {
		//let temp = T::Storage::new();
		//self.components.insert( id , Box::new(  ) );
	}
}