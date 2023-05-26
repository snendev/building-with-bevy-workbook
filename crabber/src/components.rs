use bevy::prelude::{Component, Vec3};

pub use bevy::prelude::Transform;

use crate::constants::{LEVEL_HEIGHT_F32, LEVEL_WIDTH_F32, MAX_X_F32, MAX_Y_F32, TILE_SIZE_F32};

#[derive(Component)]
pub struct Crab;
#[derive(Component)]
pub struct Raft;
#[derive(Component)]
pub struct Car;

#[derive(Clone, Copy, PartialEq)]
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

fn is_offscreen(transform: &Transform, direction: Direction) -> bool {
    let max_abs = match direction {
        Direction::Left | Direction::Right => MAX_X_F32,
        Direction::Up | Direction::Down => MAX_Y_F32,
    };
    let current_value = match direction {
        Direction::Left | Direction::Right => transform.translation.x,
        Direction::Up | Direction::Down => transform.translation.y,
    };
    let motion_vector = match direction {
        Direction::Left | Direction::Right => direction.to_vec().x,
        Direction::Up | Direction::Down => direction.to_vec().y,
    };
    // we are past the max value in the expected dimension
    current_value.abs() > max_abs
        // and we are moving farther "outside" the bounds
        && current_value.is_sign_positive() == motion_vector.is_sign_positive()
}

fn get_offset_for_loop(direction: Direction) -> Vec3 {
    let amplitude_tiles = match direction {
        Direction::Up | Direction::Down => LEVEL_HEIGHT_F32,
        Direction::Left | Direction::Right => LEVEL_WIDTH_F32,
    };
    -direction.to_vec() * TILE_SIZE_F32 * amplitude_tiles
}

#[derive(Component)]
pub struct ConstantMotor {
    pub speed: f32,
    pub direction: Direction,
}

impl ConstantMotor {
    pub fn drive_offscreen(&self, transform: &mut Transform) -> bool {
        transform.translation += self.direction.to_vec() * self.speed;
        is_offscreen(transform, self.direction)
    }

    pub fn drive_and_loop(&self, transform: &mut Transform) {
        if self.drive_offscreen(transform) {
            transform.translation += get_offset_for_loop(self.direction);
        }
    }
}

#[derive(Component, Debug)]
pub struct StepMotor {
    // this value is None if the motor is not in motion
    // if this value is Some, the inner value is the current tick count in the lifecycle of a leap
    step: Option<usize>,
}

const STEP_SPEED: f32 = 2.; // 4. pixels per tick
const MOTION_STEPS: usize = 32; // 16 ticks per step * 4 px per tick = 64 px / step, which is 1 tile

impl StepMotor {
    pub fn new() -> Self {
        StepMotor { step: None }
    }

    pub fn get_sprite_index(&self) -> usize {
        match self.step {
            Some(step) => step % 2,
            None => 0,
        }
    }

    pub fn is_running(&self) -> bool {
        self.step.is_some()
    }

    pub fn start(&mut self, transform: &mut Transform, direction: Direction) {
        *transform = transform.looking_to(Vec3::Z, direction.to_vec());
        self.step = Some(0);
    }

    pub fn drive(&mut self, transform: &mut Transform) {
        if self.is_running() {
            transform.translation += transform.local_y() * STEP_SPEED;
            self.step = self
                .step
                .map(|step| step + 1)
                .filter(|&step| step < MOTION_STEPS);
        }
    }

    pub fn reset(&mut self) {
        self.step = None;
    }
}

#[derive(Component)]
pub struct Knockout;

#[derive(Component)]
pub struct Score(pub u16);
