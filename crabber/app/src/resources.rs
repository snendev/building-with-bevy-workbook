use naia_bevy_client::CommandHistory;

use crabber_protocol::messages::InputAction;

#[derive(Resource, Default)]
pub struct TickHistory(pub CommandHistory<Vec<(Entity, InputAction)>>);
