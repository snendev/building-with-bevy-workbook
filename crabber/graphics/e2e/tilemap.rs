use bevy::prelude::{Commands, IntoSystemAppConfig, NextState, OnEnter, ResMut};

use common_e2e::Test;

use crabber_protocol::components::Level;

use crabber_graphics::{AssetsState, GraphicsPlugin as CrabGraphicsPlugin};

fn spawn_level(mut commands: Commands) {
    commands.spawn(Level::new_random());
}

fn main() {
    Test {
        label: "Test level tilemap".to_string(),
        setup: |_app| {},
        setup_graphics: |app| {
            app.add_plugin(CrabGraphicsPlugin)
                .add_system(spawn_level.in_schedule(OnEnter(AssetsState::Ready)));
        },
        frames: 60,
        check: |_, _| true,
    }
    .run();
}
