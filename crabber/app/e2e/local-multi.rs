use bevy::{
    prelude::{
        in_state, info, Commands, IntoSystemAppConfig, IntoSystemConfig, NextState, OnEnter, Query,
        Res, ResMut, Transform, With, Without,
    },
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};

use common_e2e::Test;

use crabber_protocol::{
    bundles::CrabBundle,
    components::{Crab, Knockout, Level, Position, StepMotor, TileRow},
    constants::PLAYER_Z,
    inputs::process_input,
    tick::CoreGameLoopPlugin,
};
use crabber_controls::{
    ArrowKeysControllerBundle,
    ControllerPlugin, 
    QueuedInputs, WASDControllerBundle,
}

use crabber_app::{
    components::PredictionOf, resources::SpriteSheetAssets, AppState, GraphicsPlugin as CrabGraphicsPlugin,
};

fn init(
    mut commands: Commands,
    mut state: ResMut<NextState<AppState>>,
    spritesheets: Res<SpriteSheetAssets>,
) {
    commands.spawn((
        CrabBundle::new(),
        SpriteSheetBundle {
            texture_atlas: spritesheets.crab.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(0., f32::from(TileRow(0)), PLAYER_Z),
            ..Default::default()
        },
        WASDControllerBundle::new(),
    ));
    commands.spawn((
        CrabBundle::new(),
        SpriteSheetBundle {
            texture_atlas: spritesheets.crab.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(0., f32::from(TileRow(0)), PLAYER_Z),
            ..Default::default()
        },
        ArrowKeysControllerBundle::new(),
    ));
    let level = Level::new_random();
    let (car_bundles, raft_bundles) = level.create_level_bundles();
    for bundle in car_bundles.into_iter() {
        let entity = commands.spawn(bundle).id();
        commands.entity(entity).insert(PredictionOf(entity));
    }
    for bundle in raft_bundles.into_iter() {
        let entity = commands.spawn(bundle).id();
        commands.entity(entity).insert(PredictionOf(entity));
    }
    commands.spawn(level);
    state.set(AppState::InGame);
}

pub fn process_inputs(
    // Each player entity and the associated input action for this tick
    mut inputs: ResMut<QueuedInputs>,
    mut player_query: Query<(&mut Position, &mut StepMotor), (With<Crab>, Without<Knockout>)>,
) {
    info!("{:?}", inputs.0);
    for (entity, action) in inputs.0.drain() {
        if let Ok((mut position, mut motor)) = player_query.get_mut(entity) {
            process_input(Some(action), &mut position, &mut motor);
        }
    }
}

fn main() {
    Test {
        label: "Test local multiplayer".to_string(),
        setup: |app| {
            app.add_state::<AppState>()
                .add_plugin(ControllerPlugin)
                .add_plugin(CoreGameLoopPlugin)
                .add_system(process_inputs.run_if(in_state(AppState::InGame)))
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
