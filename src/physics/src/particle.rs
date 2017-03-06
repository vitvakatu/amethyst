//! Basic particle with mass, speed and acceleration

use cgmath::Vector3;
use specs::{Component, VecStorage, Join, RunArg, System};

use amethyst::ecs::resources::Time;
use amethyst::ecs::components::LocalTransform;

pub struct Particle {
    mass: f32,
    velocity: Vector3<f32>,
    acceleration: Vector3<f32>
}

impl Component for Particle {
    type Storage = VecStorage<Particle>;
}

pub struct ParticleSystem;

impl System<()> for ParticleSystem {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (mut particles, mut locals, time) = arg.fetch(|w| {
            (w.write::<Particle>(), 
            w.write::<LocalTransform>(), 
            w.read_resource::<Time>())
            });
        let delta_time = time.delta_time.subsec_nanos() as f32 / 1.0e9;
        for (p, local) in (&mut particles, &mut locals).iter() {
            p.velocity += p.acceleration * delta_time;
            local.translation[0] += p.velocity.x;
            local.translation[1] += p.velocity.y;
            local.translation[2] += p.velocity.z;
        }
    }
}