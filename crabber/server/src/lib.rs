use bevy_app::{App, Plugin};
use bevy_ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfig, SystemSet};
use bevy_ecs::{entity::Entity, prelude::Resource};
use bevy_utils::HashMap;

use naia_bevy_server::UserKey;
use naia_bevy_server::{Plugin as ServerPlugin, ReceiveEvents, ServerConfig};

use crabber_core::TickPlugin;
use crabber_protocol::protocol;

pub mod connection;
pub mod init;
pub mod tick;

#[derive(Resource, Default)]
pub struct UserEntities {
    user_to_entity_map: HashMap<UserKey, Entity>,
    entity_to_user_map: HashMap<Entity, UserKey>,
}

impl UserEntities {
    #[allow(dead_code)]
    fn get_entity(&self, user: &UserKey) -> Option<&Entity> {
        self.user_to_entity_map.get(user)
    }

    #[allow(dead_code)]
    fn get_user(&self, entity: &Entity) -> Option<&UserKey> {
        self.entity_to_user_map.get(entity)
    }

    fn insert(&mut self, user_key: UserKey, entity: Entity) {
        self.user_to_entity_map.insert(user_key, entity);
        self.entity_to_user_map.insert(entity, user_key);
    }

    fn remove(&mut self, user: &UserKey) -> Option<Entity> {
        self.user_to_entity_map.remove(user).and_then(|entity| {
            self.entity_to_user_map.remove(&entity);
            Some(entity)
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
struct TickSet;

pub struct CrabberServerPlugin;

impl Plugin for CrabberServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ServerPlugin::new(
            ServerConfig {
                require_auth: false,
                ..Default::default()
            },
            protocol(),
        ))
        .configure_set(TickSet.in_set(ReceiveEvents))
        .init_resource::<UserEntities>()
        .add_startup_system(init::init)
        .add_plugin(TickPlugin::new(TickSet, tick::tick_events))
        .add_systems(
            (
                connection::connect_events,
                connection::disconnect_events,
                connection::error_events,
            )
                .in_set(ReceiveEvents)
                .before(TickSet),
        )
        .add_system(tick::update_entity_scopes);
    }
}
