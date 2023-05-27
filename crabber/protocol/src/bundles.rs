use bevy_ecs::prelude::Bundle;

use crate::components::{Crab, Direction, Position, Score, StepMotor, TileRow};

#[derive(Bundle)]
pub struct CrabBundle {
    crab: Crab,
    motor: StepMotor,
    position: Position,
    score: Score,
}

impl CrabBundle {
    pub fn new() -> Self {
        CrabBundle {
            crab: Crab,
            motor: StepMotor::new(),
            position: Position::new(0., f32::from(TileRow(0)), Direction::Up),
            score: Score::new(),
        }
    }
}
