use bevy::prelude::{info, EventReader, Query, ResMut};

use naia_bevy_client::{events::ClientTickEvent, Client};

use crabber_core::{EntityActionMap, TickActions};
use crabber_protocol::{channels::PlayerInputChannel, messages::InputMessage};

use crate::{components::SourceOf, resources::TickHistory};

pub fn send_and_prepare_inputs(
    mut client: Client,
    mut tick_reader: EventReader<ClientTickEvent>,
    mut tick_history: ResMut<TickHistory>,
    sources_query: Query<&SourceOf>,
    mut player_inputs: ResMut<EntityActionMap>,
) -> Vec<TickActions> {
    let mut ticks = Vec::new();

    for ClientTickEvent(client_tick) in tick_reader.iter() {
        info!("Client tick: {}", client_tick);
        let mut predicted_actions = EntityActionMap::default();

        for (entity, action) in player_inputs.0.drain() {
            // Send each command to server
            let mut input_message = InputMessage::new(Some(action));
            input_message.entity.set(&client, &entity);
            client.send_tick_buffer_message::<PlayerInputChannel, InputMessage>(
                client_tick,
                &input_message,
            );
            if let Ok(SourceOf(prediction)) = sources_query.get(entity) {
                predicted_actions.0.insert(*prediction, action);
            }
        }

        tick_history
            .0
            .insert(*client_tick, predicted_actions.clone());
        // Also proxy actions to TickPlugin
        ticks.push((*client_tick, predicted_actions));
    }

    ticks
}
