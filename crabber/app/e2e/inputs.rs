use bevy::{
    prelude::{
        in_state, Commands, IntoSystemAppConfig, IntoSystemConfig, NextState, OnEnter, Query, Res,
        ResMut, With, Without,
    },
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};

use common_e2e::Test;

use crabber_protocol::{
    components::{Crab, Knockout, Position, StepMotor},
    inputs::process_input,
};

use crabber_controls::{
    QueuedInputs, WASDControllerBundle, ControllerPlugin,
};

use crabber_app::{
    resources::SpriteSheetAssets, AppState, GraphicsPlugin as CrabGraphicsPlugin,
};

fn process_inputs(
    // Each player entity and the associated input action for this tick
    mut inputs: ResMut<QueuedInputs>,
    mut player_query: Query<(&mut Position, &mut StepMotor), (With<Crab>, Without<Knockout>)>,
) {
    for (entity, action) in inputs.0.drain() {
        if let Ok((mut position, mut motor)) = player_query.get_mut(entity) {
            process_input(Some(action), &mut position, &mut motor);
        }
    }
}

fn tick_step_motors(mut motor_query: Query<(&mut Position, &mut StepMotor)>) {
    for (mut position, mut motor) in motor_query.iter_mut() {
        motor.drive(&mut position);
    }
}

fn init(
    mut commands: Commands,
    mut state: ResMut<NextState<AppState>>,
    spritesheets: Res<SpriteSheetAssets>,
) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: spritesheets.crab.clone(),
            sprite: TextureAtlasSprite::new(0),
            ..Default::default()
        },
        StepMotor::new(),
        WASDControllerBundle::new(),
        Crab,
    ));
    state.set(AppState::InGame);
}

fn main() {
    Test {
        label: "Test inputs".to_string(),
        setup: |app| {
            app.add_state::<AppState>()
                .add_plugin(ControllerPlugin)
                .add_system(init.in_schedule(OnEnter(AppState::InGame)))
                .add_system(process_inputs.run_if(in_state(AppState::InGame)))
                .add_system(tick_step_motors.run_if(in_state(AppState::InGame)));
        },
        setup_graphics: |app| {
            app.add_plugin(CrabGraphicsPlugin);
        },
        frames: 60,
        check: |_, _| true,
    }
    .run();
}
