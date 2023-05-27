use bevy::{prelude::App, DefaultPlugins};
use crabber_app::{build, GraphicsPlugin};

fn main() {
    let mut app = App::default();
    app.add_plugins(DefaultPlugins);
    build(&mut app);
    app.add_plugin(GraphicsPlugin).run();
}
