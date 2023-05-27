use bevy::{
    prelude::{
        in_state, info, Commands, IntoSystemAppConfig, IntoSystemConfig, NextState, OnEnter, Query,
        Res, ResMut, With, Without,
    },
};

use common_e2e::Test;

use crabber_protocol::{
    bundles::CrabBundle,
    components::{Controlled, Crab, Knockout, Position, StepMotor},
    inputs::process_inputs,
    tick::CoreGameLoopPlugin,
};

use crabber_app::{
    resources::SpriteSheetAssets, AppState, ControllerPlugin, GraphicsPlugin as CrabGraphicsPlugin,
    QueuedInputs, WASDControllerBundle,
};

fn inputs(
    // Each player entity and the associated input action for this tick
    mut inputs: ResMut<QueuedInputs>,
    mut player_query: Query<
        (&mut Position, &mut StepMotor),
        (With<Crab>, Without<Knockout>, With<Controlled>),
    >,
) {
    info!("{:?}", inputs.0);
    process_inputs(inputs.0.drain().collect::<Vec<_>>(), &mut player_query);
}

fn init(
    mut commands: Commands,
    mut state: ResMut<NextState<AppState>>,
    _spritesheets: Res<SpriteSheetAssets>,
) {
    commands.spawn((CrabBundle::new(), WASDControllerBundle::new(), Controlled));
    state.set(AppState::InGame);
}

fn main() {
    Test {
        label: "Test inputs".to_string(),
        setup: |app| {
            app.add_state::<AppState>()
                .add_plugin(ControllerPlugin)
                .add_plugin(CoreGameLoopPlugin)
                .add_system(inputs.run_if(in_state(AppState::InGame)))
                .add_system(init.in_schedule(OnEnter(AppState::Connecting)));
        },
        setup_graphics: |app: &mut bevy::prelude::App| {
            app.add_plugin(CrabGraphicsPlugin);
        },
        frames: 60,
        check: |_, _| true,
    }
    .run();
}
