use bevy::{
    prelude::{App, IntoSystemAppConfig, NextState, OnEnter, ResMut},
    DefaultPlugins,
};

use crabber_graphics::{AssetsState, GraphicsPlugin};

use crabber_app::{AppState, CrabberClientPlugin};

fn on_ready(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::Connecting);
}

fn main() {
    let mut app = App::default();
    app.add_plugins(DefaultPlugins)
        .add_plugin(GraphicsPlugin)
        .add_plugin(CrabberClientPlugin)
        .add_system(on_ready.in_schedule(OnEnter(AssetsState::Ready)))
        .run();
}
