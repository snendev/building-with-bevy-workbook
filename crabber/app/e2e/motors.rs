use bevy::{
    prelude::{Commands, IntoSystemAppConfig, NextState, OnEnter, Res, ResMut},
};

use common_e2e::Test;

use crabber_protocol::{
    components::{Car, ConstantMotor, Controlled, Direction, Position, Raft},
    constants::TILE_SIZE_F32,
    tick::CoreGameLoopPlugin,
};

use crabber_app::{resources::SpriteSheetAssets, AppState, GraphicsPlugin as CrabGraphicsPlugin};

fn set_state(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::InGame);
}

fn spawn_raft(mut commands: Commands, _spritesheets: Res<SpriteSheetAssets>) {
    commands.spawn((
        Position::new(0., -TILE_SIZE_F32, Direction::Up),
        ConstantMotor::new(4., Direction::Right),
        Raft,
        Controlled,
    ));
    commands.spawn((
        Position::new(0., TILE_SIZE_F32, Direction::Up),
        ConstantMotor::new(4., Direction::Right),
        Car,
        Controlled,
    ));
}

fn main() {
    Test {
        label: "Test constant motors".to_string(),
        setup: |app| {
            app.add_state::<AppState>()
                .add_plugin(CoreGameLoopPlugin)
                .add_system(spawn_raft.in_schedule(OnEnter(AppState::InGame)))
                .add_system(set_state.in_schedule(OnEnter(AppState::Connecting)));
        },
        setup_graphics: |app| {
            app.add_plugin(CrabGraphicsPlugin);
        },
        frames: 60,
        check: |_, _| true,
    }
    .run();
}
