use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemConfigs, IntoSystemSetConfig, OnEnter, Plugin, States,
    SystemSet,
};

use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, ReceiveEvents};

use crabber_controller::ControllerPlugin;
use crabber_core::TickPlugin;
use crabber_protocol::protocol;

pub mod components;
mod connection;
mod events;
pub mod resources;
mod rollback;
mod tick;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    Waiting, // not yet ready
    Connecting,   // connecting to game
    InGame,       // in game actively
    Disconnected, // disconnected
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
struct TickSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
struct RollbackSet;

pub struct CrabberClientPlugin;

impl Plugin for CrabberClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .configure_set(TickSet.in_set(ReceiveEvents))
            .configure_set(RollbackSet.after(TickSet).in_set(ReceiveEvents))
            .init_resource::<resources::TickHistory>()
            .add_plugin(ClientPlugin::new(ClientConfig::default(), protocol()))
            .add_plugin(TickPlugin::new(TickSet, tick::send_and_prepare_inputs))
            .add_plugin(TickPlugin::new(
                RollbackSet,
                rollback::receive_update_component_events,
            ))
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
                    events::receive_insert_component_events,
                )
                    .in_set(ReceiveEvents)
                    .before(TickSet),
            );
    }
}
