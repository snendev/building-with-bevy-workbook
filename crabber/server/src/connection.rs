use bevy_ecs::{
    event::EventReader,
    system::{Commands, Query, ResMut},
};
use bevy_log::info;

use naia_bevy_server::{
    events::{ConnectEvent, DisconnectEvent, ErrorEvent},
    CommandsExt, Server,
};

use crabber_protocol::{
    bundles::CrabBundle,
    channels::PlayerAssignmentChannel,
    components::{Controlled, Crab, Level},
    messages::PlayerAssignmentMessage,
};

use crate::UserEntities;

pub fn connect_events(
    mut commands: Commands,
    mut server: Server,
    mut user_entities: ResMut<UserEntities>,
    mut event_reader: EventReader<ConnectEvent>,
    players_query: Query<&Crab>,
) {
    for ConnectEvent(user_key) in event_reader.iter() {
        let room_key = server
            .room_keys()
            .into_iter()
            .next()
            .unwrap_or_else(|| server.make_room().key());

        let address = server.user_mut(user_key).enter_room(&room_key).address();

        info!("Client connected from: {}", address);

        let num_players = players_query.into_iter().count();

        // spawn a level if we are about to spawn the second player
        if num_players == 0 {
            let level = Level::new_random();
            let (car_bundles, raft_bundles) = level.create_level_bundles();
            for bundle in car_bundles.into_iter() {
                let entity = commands
                    .spawn((bundle, Controlled))
                    .enable_replication(&mut server)
                    .id();
                server.room_mut(&room_key).add_entity(&entity);
            }
            for bundle in raft_bundles.into_iter() {
                let entity = commands
                    .spawn((bundle, Controlled))
                    .enable_replication(&mut server)
                    .id();
                server.room_mut(&room_key).add_entity(&entity);
            }
            let entity = commands
                .spawn_empty()
                .enable_replication(&mut server)
                .insert(level)
                .id();
            server.room_mut(&room_key).add_entity(&entity);
        }

        // only spawn player entities for the first two players
        if num_players < 2 {
            let entity = commands
                .spawn_empty()
                .enable_replication(&mut server)
                .insert((CrabBundle::new(), Controlled))
                .id();

            server.room_mut(&room_key).add_entity(&entity);
            user_entities.insert(*user_key, entity);

            let mut assignment_message = PlayerAssignmentMessage::new();
            assignment_message.entity.set(&server, &entity);
            server.send_message::<PlayerAssignmentChannel, PlayerAssignmentMessage>(
                user_key,
                &assignment_message,
            );
        }
    }
}

pub fn disconnect_events(
    mut commands: Commands,
    mut server: Server,
    mut user_entities: ResMut<UserEntities>,
    mut event_reader: EventReader<DisconnectEvent>,
) {
    for DisconnectEvent(user_key, user) in event_reader.iter() {
        info!("Crabber Server disconnected from: {:?}", user.address);

        if let Some(entity) = user_entities.remove(user_key) {
            let room_keys = {
                let user_ref = server.user(user_key);
                user_ref.room_keys().map(|key| *key).collect::<Vec<_>>()
            };
            for room_key in room_keys.into_iter() {
                server.room_mut(&room_key).remove_entity(&entity);
            }
            commands.entity(entity).despawn();
        }
    }
}

pub fn error_events(mut event_reader: EventReader<ErrorEvent>) {
    for ErrorEvent(error) in event_reader.iter() {
        info!("Crabber Server Error: {:?}", error);
    }
}
