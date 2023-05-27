use bevy::{
    prelude::{Commands, IntoSystemAppConfig, NextState, OnEnter, Res, ResMut},
};

use common_e2e::Test;

use crabber_app::{resources::SpriteSheetAssets, AppState, GraphicsPlugin as CrabGraphicsPlugin};
use crabber_protocol::{bundles::CrabBundle, components::Controlled};

fn set_state(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::InGame);
}

fn spawn_crab(mut commands: Commands, _spritesheets: Res<SpriteSheetAssets>) {
    commands.spawn((
        CrabBundle::new(),
        // show without tint
        Controlled,
    ));
}

fn main() {
    Test {
        label: "Test basic crab stuff".to_string(),
        setup: |app| {
            app.add_state::<AppState>()
                .add_system(spawn_crab.in_schedule(OnEnter(AppState::InGame)))
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
