use crate::*;

pub trait TSystem: Send {
    fn init(&mut self, world: &mut World) {}
    fn update(&mut self, world: &mut World, delta: f32) {}
}
