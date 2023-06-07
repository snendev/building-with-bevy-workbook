use naia_bevy_shared::Serde;

use crate::components::Direction;

#[derive(Clone, Copy, Debug, PartialEq, Serde)]
pub enum InputAction {
    Up,
    Down,
    Left,
    Right,
}

impl InputAction {
    pub fn get_direction(&self) -> Direction {
        match self {
            InputAction::Up => Direction::Up,
            InputAction::Down => Direction::Down,
            InputAction::Left => Direction::Left,
            InputAction::Right => Direction::Right,
        }
    }
}
