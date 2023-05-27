use bevy::prelude::{
    in_state, Commands, IntoSystemAppConfig, IntoSystemConfig, NextState, OnEnter, Query, ResMut,
    With, Without,
};

use common_e2e::Test;

use crabber_protocol::{
    bundles::CrabBundle,
    components::{Controlled, Crab, Knockout, Level, Position, StepMotor},
    inputs::process_inputs,
    tick::CoreGameLoopPlugin,
};

use crabber_app::{
    AppState, ControllerPlugin, GraphicsPlugin as CrabGraphicsPlugin, QueuedInputs,
    WASDControllerBundle,
};

fn set_state(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::InGame);
}

fn inputs(
    // Each player entity and the associated input action for this tick
    mut inputs: ResMut<QueuedInputs>,
    mut player_query: Query<
        (&mut Position, &mut StepMotor),
        (With<Crab>, Without<Knockout>, With<Controlled>),
    >,
) {
    process_inputs(inputs.0.drain().collect::<Vec<_>>(), &mut player_query);
}

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
    commands.spawn((CrabBundle::new(), WASDControllerBundle::new(), Controlled));
}

fn main() {
    Test {
        label: "Test common full game".to_string(),
        setup: |app| {
            app.add_state::<AppState>()
                .add_plugin(ControllerPlugin)
                .add_plugin(CoreGameLoopPlugin)
                .add_system(inputs.run_if(in_state(AppState::InGame)))
                .add_system(set_state.in_schedule(OnEnter(AppState::Connecting)))
                .add_system(init.in_schedule(OnEnter(AppState::InGame)));
        },
        setup_graphics: |app| {
            app.add_plugin(CrabGraphicsPlugin);
        },
        frames: 60,
        check: |_, _| true,
    }
    .run();
}
