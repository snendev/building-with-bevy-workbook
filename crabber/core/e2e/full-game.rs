use bevy_app::prelude::IntoSystemAppConfig;
use bevy_ecs::{
    prelude::{OnEnter, Res},
    schedule::SystemSet,
    system::Commands,
};

use common_e2e::Test;

use crabber_controller::{components::Controller, ControllerPlugin};
use crabber_graphics::{AssetsState, GraphicsPlugin};
use crabber_protocol::{
    bundles::CrabBundle,
    components::{Controlled, Level},
};

use crabber_core::{EntityActionMap, TickActions, TickPlugin};

fn read_actions(actions: Res<EntityActionMap>) -> Vec<TickActions> {
    vec![(0, actions.clone())]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
struct TickSet;

// must spawn entities after entering new state to ensure that `Added` components are detected in graphics
fn init(mut commands: Commands) {
    // spawn level
    let level = Level::new_random();
    let (car_bundles, raft_bundles) = level.create_level_bundles();
    for bundle in car_bundles.into_iter() {
        commands.spawn((bundle, Controlled));
    }
    for bundle in raft_bundles.into_iter() {
        commands.spawn((bundle, Controlled));
    }
    commands.spawn(level);

    // spawn crab
    commands.spawn((CrabBundle::new(), Controller::Keyboard(0), Controlled));
}

fn main() {
    Test {
        label: "Test common full game".to_string(),
        setup: |app| {
            app.add_plugin(TickPlugin::new(TickSet, read_actions))
                .add_plugin(ControllerPlugin)
                .add_system(init.in_schedule(OnEnter(AssetsState::Ready)));
        },
        setup_graphics: |app| {
            app.add_plugin(GraphicsPlugin);
        },
        frames: 60,
        check: |_, _| true,
    }
    .run();
}
