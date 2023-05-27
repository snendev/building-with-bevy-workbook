use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemConfigs, IntoSystemSetConfig, OnEnter, States, SystemSet,
};

use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, ReceiveEvents};

use crabber_protocol::protocol;

pub mod components;
mod connection;
mod controller;
pub use controller::{
    ArrowKeysControllerBundle, ControllerPlugin, QueuedInputs, WASDControllerBundle,
};
mod events;
mod graphics;
pub use graphics::GraphicsPlugin;
use resources::TickHistory;
pub mod resources;
mod tick;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    Loading, // loading assets
    Connecting,   // connecting to game
    InGame,       // in game actively
    Disconnected, // disconnected
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
struct Tick;

pub fn build(app: &mut App) -> &mut App {
    app.add_state::<AppState>()
        .configure_set(Tick.in_set(ReceiveEvents))
        .init_resource::<TickHistory>()
        .add_plugin(ClientPlugin::new(ClientConfig::default(), protocol()))
        .add_plugin(ControllerPlugin)
        // try to initiate a connection once we enter the "InGame" state
        .add_system(connection::inititate_connection.in_schedule(OnEnter(AppState::Connecting)))
        // react to any connection, disconnection, rejection events from server
        .add_systems(
            (
                connection::connection_events,
                connection::disconnection_events,
                connection::rejection_events,
                events::receive_entity_assignment_message,
            )
                .in_set(ReceiveEvents)
                .before(Tick),
        )
        // also handle tick events, and ensure these occur in order
        .add_systems(
            (
                events::receive_insert_component_events,
                tick::tick,
                events::receive_update_component_events,
            )
                .chain()
                .in_set(Tick),
        )
}
