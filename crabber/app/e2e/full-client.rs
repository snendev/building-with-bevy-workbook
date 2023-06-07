use bevy::prelude::{IntoSystemAppConfig, NextState, OnEnter, ResMut};
use common_e2e::Test;
use crabber_app::{AppState, CrabberClientPlugin};
use crabber_graphics::{AssetsState, GraphicsPlugin};

fn on_ready(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::Connecting);
}

fn main() {
    Test {
        label: "Test full client".to_string(),
        setup: |app| {
            app.add_plugin(CrabberClientPlugin)
                .add_system(on_ready.in_schedule(OnEnter(AssetsState::Ready)))
                .run();
        },
        setup_graphics: |app| {
            app.add_plugin(GraphicsPlugin);
        },
        frames: 60,
        check: |_, _| true,
    }
    .run();
}
