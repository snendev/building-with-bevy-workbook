use bevy::{
    prelude::{
        in_state, info, Commands, IntoSystemAppConfig, IntoSystemConfig, NextState, OnEnter, Query,
        Res, ResMut, With, Without,
    },
};

use common_e2e::Test;

use crabber_protocol::{
    bundles::CrabBundle,
    components::{Controlled, Crab, Knockout, Level, Position, StepMotor},
    inputs::process_inputs,
    tick::CoreGameLoopPlugin,
};

use crabber_app::{
    components::PredictionOf, resources::SpriteSheetAssets, AppState, ArrowKeysControllerBundle,
    ControllerPlugin, GraphicsPlugin as CrabGraphicsPlugin, QueuedInputs, WASDControllerBundle,
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
    commands.spawn((
        CrabBundle::new(),
        ArrowKeysControllerBundle::new(),
        Controlled,
    ));
    let level = Level::new_random();
    let (car_bundles, raft_bundles) = level.create_level_bundles();
    for bundle in car_bundles.into_iter() {
        let entity = commands.spawn(bundle).id();
        commands
            .entity(entity)
            .insert((PredictionOf(entity), Controlled));
    }
    for bundle in raft_bundles.into_iter() {
        let entity = commands.spawn(bundle).id();
        commands
            .entity(entity)
            .insert((PredictionOf(entity), Controlled));
    }
    commands.spawn(level);
    state.set(AppState::InGame);
}

fn main() {
    Test {
        label: "Test local multiplayer".to_string(),
        setup: |app| {
            app.add_state::<AppState>()
                .add_plugin(ControllerPlugin)
                .add_plugin(CoreGameLoopPlugin)
                .add_system(inputs.run_if(in_state(AppState::InGame)))
                .add_system(init.in_schedule(OnEnter(AppState::Connecting)));
        },
        setup_graphics: |app| {
            app.add_plugin(CrabGraphicsPlugin);
        },
        frames: 60,
        check: |_, _| true,
    }
    .run();
}
