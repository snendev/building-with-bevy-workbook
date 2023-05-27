use bevy::prelude::{info, EventReader, NextState, ResMut};

use naia_bevy_client::{
    events::{ConnectEvent, DisconnectEvent, RejectEvent},
    transport::webrtc,
    Client,
};

use crate::AppState;

pub fn inititate_connection(mut client: Client) {
    // create a socket
    let socket = webrtc::Socket::new("http://127.0.0.1:14191", client.socket_config());
    client.connect(socket);
}

pub fn connection_events(
    mut event_reader: EventReader<ConnectEvent>,
    client: Client,
    mut state: ResMut<NextState<AppState>>,
) {
    for _event in event_reader.iter() {
        info!("Client connected to: {:?}", client.server_address());
        // enter the "InGame" state
        state.set(AppState::InGame);
    }
}

pub fn disconnection_events(
    client: Client,
    mut state: ResMut<NextState<AppState>>,
    mut event_reader: EventReader<DisconnectEvent>,
) {
    for _event in event_reader.into_iter() {
        info!("Client disconnected from: {:?}", client.server_address());
        // reset to loading state for now
        // eventually, we could use this to show an alert supporting user actions
        // or handle some sort of reconnection
        state.set(AppState::Connecting);
    }
}

pub fn rejection_events(mut event_reader: EventReader<RejectEvent>) {
    for _ in event_reader.iter() {
        info!("Client rejected from connecting to Server");
    }
}
