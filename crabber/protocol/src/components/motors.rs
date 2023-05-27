use bevy_ecs::prelude::Component;
use bevy_math::prelude::Vec3;

use naia_bevy_shared::{Property, Replicate};

use crate::{
    components::{Direction, Position},
    constants::{LEVEL_HEIGHT_F32, LEVEL_WIDTH_F32, MAX_X_F32, MAX_Y_F32, TILE_SIZE_F32},
};

fn is_offscreen(position: &Position, direction: Direction) -> bool {
    let max_abs = match direction {
        Direction::Left | Direction::Right => MAX_X_F32,
        Direction::Up | Direction::Down => MAX_Y_F32,
    };
    let current_value = match direction {
        Direction::Left | Direction::Right => *position.x,
        Direction::Up | Direction::Down => *position.y,
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

#[derive(Component, Replicate)]
pub struct ConstantMotor {
    pub speed: Property<f32>,
    pub direction: Property<Direction>,
}

impl ConstantMotor {
    pub fn new(speed: f32, direction: Direction) -> Self {
        Self::new_complete(speed, direction)
    }

    pub fn drive_offscreen(&self, position: &mut Position) -> bool {
        let delta = self.direction.to_vec() * *self.speed;
        *position.x += delta.x;
        *position.y += delta.y;
        is_offscreen(position, *self.direction)
    }

    pub fn drive_and_loop(&self, position: &mut Position) {
        if self.drive_offscreen(position) {
            let delta = get_offset_for_loop(*self.direction);
            *position.x += delta.x;
            *position.y += delta.y;
        }
    }
}

#[derive(Component, Replicate)]
pub struct StepMotor {
    // this value is None if the motor is not in motion
    // if this value is Some, the inner value is the current tick count in the lifecycle of a leap
    step: Property<Option<usize>>,
}

const STEP_SPEED: f32 = 2.; // 4. pixels per tick
const MOTION_STEPS: usize = 32; // 16 ticks per step * 4 px per tick = 64 px / step, which is 1 tile

impl StepMotor {
    pub fn new() -> Self {
        Self::new_complete(None)
    }

    pub fn is_running(&self) -> bool {
        self.step.is_some()
    }

    pub fn start(&mut self, position: &mut Position, direction: Direction) {
        *position.direction = direction;
        *self.step = Some(0);
    }

    pub fn get_sprite_index(&self) -> usize {
        match *self.step {
            Some(step) => step % 2,
            None => 0,
        }
    }

    pub fn drive(&mut self, position: &mut Position) {
        if self.is_running() {
            position.move_forward(STEP_SPEED);
            *self.step = self
                .step
                .map(|step| step + 1)
                .filter(|&step| step < MOTION_STEPS);
        }
    }

    pub fn reset(&mut self) {
        *self.step = None;
    }
}
