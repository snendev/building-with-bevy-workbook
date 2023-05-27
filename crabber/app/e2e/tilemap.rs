use bevy::prelude::{Commands, IntoSystemAppConfig, NextState, OnEnter, ResMut};

use common_e2e::Test;

use crabber_protocol::components::Level;

use crabber_app::{AppState, GraphicsPlugin as CrabGraphicsPlugin};

fn set_state(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::InGame);
}

fn spawn_level(mut commands: Commands) {
    commands.spawn(Level::new_random());
}

fn main() {
    Test {
        label: "Test level tilemap".to_string(),
        setup: |app| {
            app.add_state::<AppState>()
                .add_system(set_state.in_schedule(OnEnter(AppState::Connecting)))
                .add_system(spawn_level.in_schedule(OnEnter(AppState::InGame)));
        },
        setup_graphics: |app| {
            app.add_plugin(CrabGraphicsPlugin);
        },
        frames: 60,
        check: |_, _| true,
    }
    .run();
}
