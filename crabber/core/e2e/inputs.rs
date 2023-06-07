use bevy_app::{App, IntoSystemAppConfig};
use bevy_ecs::prelude::{Commands, OnEnter, Res};

use bevy_ecs::schedule::SystemSet;
use common_e2e::Test;

use crabber_controller::{components::Controller, ControllerPlugin};
use crabber_graphics::{AssetsState, GraphicsPlugin as CrabGraphicsPlugin};
use crabber_protocol::{bundles::CrabBundle, components::Controlled};

use crabber_core::{EntityActionMap, TickActions, TickPlugin};

fn init(mut commands: Commands) {
    commands.spawn((CrabBundle::new(), Controller::Keyboard(0), Controlled));
}

fn read_actions(actions: Res<EntityActionMap>) -> Vec<TickActions> {
    vec![(0, actions.clone())]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
struct TickSet;

fn main() {
    Test {
        label: "Test inputs".to_string(),
        setup: |app| {
            app.add_plugin(TickPlugin::new(TickSet, read_actions))
                .add_plugin(ControllerPlugin)
                .add_system(init.in_schedule(OnEnter(AssetsState::Ready)));
        },
        setup_graphics: |app: &mut App| {
            app.add_plugin(CrabGraphicsPlugin);
        },
        frames: 60,
        check: |_, _| true,
    }
    .run();
}
