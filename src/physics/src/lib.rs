#![crate_name = "amethyst_physics"]
#![crate_type = "lib"]

extern crate cgmath;
extern crate specs;

mod particle;

pub use particle::Particle;