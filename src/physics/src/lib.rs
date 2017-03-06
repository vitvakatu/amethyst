#![crate_name = "amethyst_physics"]
#![crate_type = "lib"]

extern crate cgmath;
extern crate specs;

mod particle;

pub type Delta = f32;

pub use particle::Particle;