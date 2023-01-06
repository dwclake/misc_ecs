use std::collections::HashMap;
use crate::ecs::component::ComponentID;
use crate::ecs::entity::EntityBuilder;
use crate::ecs::system::SystemID;
use crate::prelude::*;

///
///
/// # Examples
///
/// ```
///
/// ```
///
pub struct World {
	pub entity_manager: EntityManager,
	pub components: HashMap< ComponentID, HashStore<Box<dyn Component<Type = f32>>>>,
	pub systems: HashMap< SystemID, Box<dyn System<SystemData = ()>>>,
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
			components: HashMap::new(),
			systems: HashMap::new(),
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
	/// use std::ops::Add;
	/// use misc_ecs::ecs::component::ComponentID;
	/// use misc_ecs::ecs::store::Storage;
	/// use misc_ecs::prelude::*;
	///
	/// struct Pos<T>(T);
	///
	/// impl<T: Add> Component for Pos<T> {
	/// 	type Type = T;
	/// }
	/// let mut world = World::new();
	///
	/// let mut x = HashStore::new();
	/// let ent = world.entity_manager.next();
	/// x.add( ent, Box::new( Pos(42.0) ) );
	///
	///
	/// let acc = ComponentID::new();
	/// //world.register( acc, x );
	///
	///
	/// //x.add( ent, 42.0 );
	/// assert_eq!( x.len(), 1);
	/// assert!( &x.get( ent ).unwrap().0 - 42.0 < 0.00001 );
	/// ```
	///
	pub fn register<T>(&mut self, component_id: ComponentID, component: HashStore<Box<dyn Component<Type = f32>>> ) {
		self.components.insert( component_id , component );
	}
}