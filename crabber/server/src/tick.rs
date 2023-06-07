use bevy_ecs::event::EventReader;

use naia_bevy_server::{events::TickEvent, Server};

use crabber_protocol::{channels::PlayerInputChannel, messages::InputMessage};

use crabber_core::{EntityActionMap, TickActions};

pub fn tick_events(
    mut server: Server,
    mut tick_reader: EventReader<TickEvent>,
) -> Vec<TickActions> {
    let mut tick_actions = Vec::new();

    for TickEvent(server_tick) in tick_reader.iter() {
        let mut player_actions = EntityActionMap::default();
        let mut messages = server.receive_tick_buffer_messages(server_tick);
        for (_user_key, command) in messages.read::<PlayerInputChannel, InputMessage>() {
            let Some(entity) = command.entity.get(&server) else { continue };
            if let Some(action) = command.action {
                player_actions.0.insert(entity, action);
            }
        }
        tick_actions.push((*server_tick, player_actions));
    }

    tick_actions
}

pub fn update_entity_scopes(mut server: Server, mut tick_reader: EventReader<TickEvent>) {
    if !tick_reader.iter().count() != 0 {
        // Update entity scopes
        for (_room_key, user_key, entity) in server.scope_checks() {
            server.user_scope(&user_key).include(&entity);
        }
    }
}
