use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::default::Default;

#[derive(Debug, Clone, Copy, Component, Default)]
struct IsWheel;

#[derive(Debug, Clone, Copy, Component)]
struct WheelPhysics {
    /// used to calculate the grip between the wheel and surface
    grip_factor: f32,

    /// Changes the grip factor
    /// Value in [0, 1] range
    usage: f32,
}

impl Default for WheelPhysics {
    fn default() -> Self {
        Self { 
            grip_factor: 1.0, 
            usage: 0.0,
        }
    }
}

#[derive(Clone, Default, Bundle)]
pub struct Wheel {
    wheel_marker: IsWheel,
    physics: WheelPhysics,
    area: Collider,
    sensor: Sensor,

    #[bundle]
    transform: TransformBundle,
}

impl Wheel {
    pub fn position(mut self, x: f32, y: f32, z: f32) -> Self {
        self.transform = TransformBundle::from_transform(Transform::from_xyz(x, y, z));

        return self;
    }

    pub fn size(mut self, hx: f32, hy: f32) -> Self {
        self.area = Collider::cuboid(hx, hy);

        return self;
    }
}

#[derive(Default, Component)]
pub(super) struct Wheels(Vec<Wheel>);

impl Wheels {
    pub fn new(count: usize) -> Self {
        Self(vec![Wheel::default(); count])
    }
}

