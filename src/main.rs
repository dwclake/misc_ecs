use misc_ecs::{ gen::*, store::* };

pub struct Attributes {
	pub mass: f64,
	pub radius: f64,
}

fn main() {
	
	let mut gen_manager = EntityManager::new();
	let mut pos = HashStore::new();
	let mut vel = HashStore::new();
	let mut acc = HashStore::new();
	let mut pass = 0;
	
	loop {
		// create one entity per loop (choice not requirement)
		if gen_manager.len() < 100 {
			let entity = gen_manager.next();
			pos.add(entity, 5 );
			vel.add(entity, 5 );
			acc.add(entity, 5 );
		} else { break }
		
		let _x = format!("Pass = {}", pass);
		pass += 1;
		
		std::thread::sleep(std::time::Duration::from_millis(30));
	}
	
	dbg!( gen_manager );
}
