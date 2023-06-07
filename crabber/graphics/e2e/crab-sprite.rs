use bevy::prelude::{Commands, IntoSystemAppConfig, OnEnter};

use common_e2e::Test;

use crabber_protocol::{bundles::CrabBundle, components::Controlled};

use crabber_graphics::{AssetsState, GraphicsPlugin as CrabGraphicsPlugin};

fn spawn_crab(mut commands: Commands) {
    commands.spawn((
        CrabBundle::new(),
        // show without tint
        Controlled,
    ));
}

fn main() {
    Test {
        label: "Test basic crab stuff".to_string(),
        setup: |_app| {},
        setup_graphics: |app| {
            app.add_plugin(CrabGraphicsPlugin).add_system(spawn_crab.in_schedule(OnEnter(AssetsState::Ready)));
        },
        frames: 60,
        check: |_, _| true,
    }
    .run();
}
