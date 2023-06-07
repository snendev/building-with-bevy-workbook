use naia_bevy_shared::{EntityProperty, Message};

use crate::inputs::InputAction;

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
