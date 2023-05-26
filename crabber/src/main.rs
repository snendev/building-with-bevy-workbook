use bevy::prelude::{App, DefaultPlugins};

use crabber::{AppState, CoreGameLoopPlugin, GraphicsPlugin as CrabGraphicsPlugin, InputPlugin};

fn main() {
    App::default()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(CrabGraphicsPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(CoreGameLoopPlugin)
        .run();
}
