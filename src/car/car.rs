use bevy::{prelude::*, ecs::system::EntityCommands};
use bevy_rapier2d::prelude::*;

use super::wheel::Wheels;

#[derive(Debug, Default, Component)]
struct CarInternals {
    pub engine_power: f32,
    pub break_power: f32,

    /// distribution of `engine_power` between front and back wheels in range [-1, 1],
    /// where -1 is 100% power to the back wheels, and 1 is 100% power to the front wheels
    pub power_bias: f32,

    /// distribution of `break_power` between front and back wheels in range [-1, 1]
    /// where -1 is 100% power to the back wheels, and 1 is 100% power to the front wheels
    pub break_bias: f32,
}

impl CarInternals {
    fn new(engine_power: f32, break_power: f32, power_bias: f32, break_bias: f32) -> Self {
        Self {
            engine_power,
            break_power,
            power_bias,
            break_bias,
        }
    }
}

#[derive(Bundle)]
struct CarBundle {
    pub internals: CarInternals,
    collider: Collider,
    rigidbody: RigidBody,

    pub wheels: Wheels,

    #[bundle]
    transform: TransformBundle,
}

impl Default for CarBundle {
    fn default() -> Self {
        Self {
            rigidbody: RigidBody::Dynamic,
            wheels: Wheels::new(4),
            ..default()
        }
    }
}

impl CarBundle {
    pub fn position(mut self, x: f32, y: f32, z: f32) -> Self {
        self.transform = TransformBundle::from_transform(Transform::from_xyz(x, y, z));

        return self;
    }

    pub fn size(mut self, hx: f32, hy: f32) -> Self {
        todo!();
    }

    pub fn car_internals(mut self, internals: CarInternals) -> Self {
        self.internals = internals;

        return self;
    }
}

