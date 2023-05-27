use bevy_ecs::prelude::Component;
use bevy_math::prelude::Vec3;

use naia_bevy_shared::{Property, Replicate, Serde};

#[derive(Clone, Copy, PartialEq, Serde)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn to_vec(&self) -> Vec3 {
        match self {
            Direction::Up => Vec3::Y,
            Direction::Right => Vec3::X,
            Direction::Down => Vec3::NEG_Y,
            Direction::Left => Vec3::NEG_X,
        }
    }
}

#[derive(Component, Replicate)]
pub struct Position {
    pub x: Property<f32>,
    pub y: Property<f32>,
    pub direction: Property<Direction>,
}

impl Position {
    pub fn new(x: f32, y: f32, direction: Direction) -> Self {
        Position::new_complete(x, y, direction)
    }

    pub fn move_direction(&mut self, delta: f32, direction: Direction) {
        match direction {
            Direction::Up => {
                *self.y += delta;
            }
            Direction::Down => {
                *self.y -= delta;
            }
            Direction::Right => {
                *self.x += delta;
            }
            Direction::Left => {
                *self.x -= delta;
            }
        }
    }

    pub fn move_forward(&mut self, delta: f32) {
        self.move_direction(delta, *self.direction);
    }
}
