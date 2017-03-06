#![crate_name = "amethyst_physics"]
#![crate_type = "lib"]

extern crate amethyst;
extern crate cgmath;
extern crate specs;

mod particle;

pub type Delta = f32;

pub use particle::Particle;
pub use particle::ParticleSystem;