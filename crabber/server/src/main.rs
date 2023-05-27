use bevy_app::{App, ScheduleRunnerPlugin, ScheduleRunnerSettings};
use bevy_core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin};
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_ecs::{entity::Entity, prelude::Resource};
use bevy_log::{info, LogPlugin};
use std::collections::HashMap;
use std::time::Duration;

use naia_bevy_server::UserKey;
use naia_bevy_server::{Plugin as ServerPlugin, ReceiveEvents, ServerConfig};

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

fn main() {
    info!("Starting up Crabber server...");

    App::default()
        .add_plugin(TaskPoolPlugin::default())
        .add_plugin(TypeRegistrationPlugin::default())
        .add_plugin(FrameCountPlugin::default())
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_millis(3)))
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(LogPlugin::default())
        .add_plugin(ServerPlugin::new(
            ServerConfig {
                require_auth: false,
                ..Default::default()
            },
            protocol(),
        ))
        .init_resource::<UserEntities>()
        .add_startup_system(init::init)
        .add_systems(
            (
                connection::connect_events,
                connection::disconnect_events,
                connection::error_events,
                tick::tick_events,
            )
                .in_set(ReceiveEvents),
        )
        .run();
}
