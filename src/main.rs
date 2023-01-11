use misc_ecs::prelude::*;

#[derive(Debug, Copy, Clone)]
struct Pos(f32);

impl Component for Pos {
	type Storage = HashStore<Self>;
}

fn lim( _: Entity, x: &mut Pos ) {
	x.0 = x.0.clamp( 0.0, 100.0 );
}

fn main() {

	let mut world = World::new();

	let mut x = HashStore::new();
	let ent = world.entity_manager_mut().next();
	x.add( ent, Pos(402.0) );

	x.for_each_mut( | entity, data| {
		lim ( entity,data );
	});

	x.for_each_mut( lim );

	let acc = ComponentID::new();
	world.register::<Pos>( acc );

	for (_x, y) in x {
 		println!("{:?}", y.0);
	}
}
