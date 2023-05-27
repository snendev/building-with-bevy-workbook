use naia_bevy_shared::{EntityProperty, Message, Serde};

use crate::components::Direction;

#[derive(Message)]
pub struct PlayerAssignmentMessage {
    pub entity: EntityProperty,
}

impl PlayerAssignmentMessage {
    pub fn new() -> Self {
        PlayerAssignmentMessage {
            entity: EntityProperty::new_empty(),
        }
    }
}

impl Default for PlayerAssignmentMessage {
    fn default() -> Self {
        Self::new()
    }
}

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

#[derive(Message)]
pub struct InputMessage {
    pub entity: EntityProperty,
    pub action: Option<InputAction>,
}

impl InputMessage {
    pub fn new(action: Option<InputAction>) -> Self {
        InputMessage {
            entity: EntityProperty::new_empty(),
            action,
        }
    }
}
