use bevy_ecs::prelude::*;
use glam::*;
use raylib::prelude::*;
use resources::*;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};
use system::*;

pub mod prelude;
mod resources;
mod system;

pub fn run<const L: usize>(mut systems: [Box<dyn TSystem>; L]) {
    let world = Arc::new(Mutex::new(World::new()));
    let (mut rh, thread) = raylib::init()
        .vsync()
        .title("Dino Engine")
        .size(1280, 720)
        .build();

    {
        let mut world = world.lock().unwrap();
        world.insert_resource(FixedDelta(1.0 / 120.0));
    }

    for system in systems.iter_mut() {
        let mut world = world.lock().unwrap();
        system.init(&mut world);
    }

    let world1 = world.clone();
    let handle = thread::spawn(move || {
        let mut elapsed = Instant::now();

        loop {
            let mut world = world1.lock().unwrap();

            if world.get_resource::<ShouldClose>().is_some() {
                return;
            }
            let delta = world.get_resource::<FixedDelta>().unwrap().0;

            if elapsed.elapsed().as_secs_f32() < delta {
                continue;
            }

            elapsed = Instant::now();

            for system in systems.iter_mut() {
                system.update(&mut world, delta);
            }
        }
    });

    let world = world.lock().unwrap();
    while !rh.window_should_close() {
        let mut rdh = rh.begin_drawing(&thread);

        rdh.clear_background(Color::BLACK);

        println!("{:#?}", world);
    }
    handle.join().unwrap();
}
