//! Basic particle with mass, speed and acceleration

use cgmath::Vector3;
use specs::{Component, VecStorage};

pub struct Particle {
    mass: f32,
    position: Vector3<f32>,
    velocity: Vector3<f32>,
    acceleration: Vector3<f32>
}

impl Component for Particle {
    type Storage = VecStorage<Particle>;
}