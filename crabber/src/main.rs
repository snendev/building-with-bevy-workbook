use bevy::prelude::{App, DefaultPlugins};

use crabber::{AppState, GraphicsPlugin as CrabGraphicsPlugin};

fn main() {
    App::default()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(CrabGraphicsPlugin)
        .run();
}
