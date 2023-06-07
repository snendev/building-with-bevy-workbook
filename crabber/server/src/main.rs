use std::time::Duration;

use bevy_app::{App, ScheduleRunnerPlugin, ScheduleRunnerSettings};
use bevy_core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin};
use bevy_log::{info, LogPlugin};

use crabber_server::CrabberServerPlugin;

fn main() {
    info!("Starting up Crabber server...");

    App::default()
        .add_plugin(TaskPoolPlugin::default())
        .add_plugin(TypeRegistrationPlugin::default())
        .add_plugin(FrameCountPlugin::default())
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_millis(3)))
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(LogPlugin::default())
        .add_plugin(CrabberServerPlugin)
        .run();
}
