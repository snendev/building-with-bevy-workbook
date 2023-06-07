use bevy::prelude::Resource;

use crabber_core::EntityActionMap;
use naia_bevy_client::CommandHistory;

#[derive(Resource, Default)]
pub struct TickHistory(pub CommandHistory<EntityActionMap>);
