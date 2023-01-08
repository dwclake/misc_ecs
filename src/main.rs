use misc_ecs::prelude::*;

pub struct Attributes {
	pub mass: f64,
	pub radius: f64,
}

fn main() {
	
	#[derive(Debug, Copy, Clone)]
	struct Pos(f32);
	
	impl Component for Pos {
	    type Storage = HashStore<Self>;
	}

	    let mut world = World::new();
	
	    let mut x = HashStore::new();
	    let ent = world.entity_manager_mut().next();
	    x.add( ent, Box::new( Pos(42.0) ) );
	
	    let acc = ComponentID::new();
	    world.register::<Pos>( acc );

	for (_x, y) in x {
 		println!("{:?}", y.0);
	}
}
