use miscmath;
use miscmath::{CoordSystem::*, vector::Vec2};
use phys_engine::ecs::{gen::*, store::*, systems, data::Attributes};

use std::ops::Range;
use raylib::prelude::*;

fn lim( v: &mut Vec2, rng: &Range<f64> ) {
	if v.x > rng.end || v.x < rng.start { v.x = v.x.clamp( rng.start, rng.end ) };
	if v.y > rng.end || v.y < rng.start { v.y = v.y.clamp( rng.start, rng.end ) };
}

fn main() {
	
	const SCREEN_SIZE: ( i32,i32 ) = ( 640, 480 );
	let (wf, hf) = ( SCREEN_SIZE.0 as f64, SCREEN_SIZE.1 as f64 );
	
	/*let (ch_sender, ch_receiver) = std::sync::mpsc::channel( );
	std::thread::spawn( move || {
		let stdin = std::io::stdin();
		for key in stdin.keys() {
			ch_sender.send( key ).expect( "Key send failed" );
		}
	});*/
	
	let ( mut rl, thread ) = init()
		.size( SCREEN_SIZE.0, SCREEN_SIZE.1 )
		.title( "phys_engine" )
		.build();
	
	let mut gen_manager = GenManager::new();
	let mut atr = VecStore::new();
	let mut pos = VecStore::new();
	let mut vel = VecStore::new();
	let mut acc = VecStore::new();
	let mut pass = 0;
	
	while !rl.window_should_close() {
		
		let mut display = rl.begin_drawing( &thread );
		display.clear_background( Color::WHITE );
		
		// create one entity per loop (choice not requirement)
		if gen_manager.len() < 100 {
			let entity = gen_manager.next();
			atr.add(entity, Attributes { mass: 12.0, radius: 5.0, color: Color::RED });
			pos.add(entity, Vec2::create_random(&(0.0..wf), &(0.0..hf), &CARTESIAN));
			vel.add(entity, Vec2::create_random(&(-0.1..0.1), &(-0.1..0.1), &CARTESIAN));
			acc.add(entity, Vec2::new());
		}
		systems::accel_sys( &acc, &mut vel );
		systems::move_sys( &vel, &mut pos );
		systems::collision_sys( &acc, &vel, &pos, &mut atr );
		systems::death_sys( SCREEN_SIZE, &mut gen_manager, &mut acc, &mut vel, &mut pos, &mut atr );
		systems::render_sys( &mut display,  SCREEN_SIZE, &pos, &atr );
		
		vel.for_each_mut( | _gen, v| lim( v, &(-10.5..10.5) ) );
		
		acc.for_each_mut( | _gen, ac| {
			ac.add( &Vec2::create_random( &(-0.5..0.5), &(-0.5..0.5), &CARTESIAN ) );
		});
		
		acc.for_each_mut( | _gen, ac| lim( ac, &(-1.1..1.1)));
		
		let x = format!( "Pass = {}", pass );
		display.draw_text( &x, 12, 12, 20, Color::BLACK );
		pass += 1;
		
		/*while let Ok( Ok( key ) ) = ch_receiver.try_recv() {
			match key {
				Key::Char( 'q' ) => return,
				// here handle any key presses to make the game do stuff
				_ => {},
			}
		}*/
		
		std::thread::sleep( std::time::Duration::from_millis( 30 ) );
	}
    
}
