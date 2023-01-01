use misc_ecs::{ gen::*, store::* };
use std::ops::Range;
use rand::random;

pub struct Attributes {
	pub mass: f64,
	pub radius: f64,
}

fn main() {
	
	let mut gen_manager = GenManager::new();
	let mut pos = VecStore::new();
	let mut vel = VecStore::new();
	let mut acc = VecStore::new();
	let mut pass = 0;
	
	loop {
		// create one entity per loop (choice not requirement)
		if gen_manager.len() < 100 {
			let entity = gen_manager.next();
			pos.add(entity, 5 );
			vel.add(entity, 5 );
			acc.add(entity, 5 );
		}
		
		acc.for_each_mut(|_gen, ac| {
			*ac += random::<i32>();
		});
		
		let _x = format!("Pass = {}", pass);
		pass += 1;
		
		std::thread::sleep(std::time::Duration::from_millis(3000));
	}
}
