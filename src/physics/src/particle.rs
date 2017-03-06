//! Basic particle with mass, speed and acceleration

use cgmath::Vector3;
use specs::{Component, VecStorage, Join, Entity, RunArg, System};

pub struct Particle {
    mass: f32,
    position: Vector3<f32>,
    velocity: Vector3<f32>,
    acceleration: Vector3<f32>
}

impl Component for Particle {
    type Storage = VecStorage<Particle>;
}

pub struct ParticleSystem;

impl System<super::Delta> for ParticleSystem {
    fn run(&mut self, arg: RunArg, time: super::Delta) {
        let mut particles = arg.fetch(|w| w.write::<Particle>());
        for p in (&mut particles).iter() {
            p.position += p.velocity * time;
            p.velocity += p.acceleration * time;
        }
    }
}