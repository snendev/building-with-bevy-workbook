use bevy_ecs::prelude::Component;

use naia_bevy_shared::{Property, Replicate};

#[derive(Component, Replicate)]
pub struct Score {
    pub value: Property<u16>,
}

impl Score {
    pub fn new() -> Self {
        Self::new_complete(0)
    }
}
