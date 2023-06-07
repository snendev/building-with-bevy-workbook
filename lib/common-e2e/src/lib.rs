use std::thread;

use bevy::{app::App, winit::WinitSettings, DefaultPlugins};

use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn on_main_thread() -> bool {
    println!("thread name: {}", thread::current().name().unwrap());
    matches!(thread::current().name(), Some("main"))
}

pub struct Test<A> {
    pub label: String,
    pub setup: fn(&mut App) -> A,
    pub setup_graphics: fn(&mut App),
    pub frames: u64,
    pub check: fn(&App, A) -> bool,
}

impl<A> Test<A> {
    pub fn run(&self) {
        let on_main_thread = on_main_thread();
        assert!(
            on_main_thread,
            "Integration test must be run on main thread!"
        );
        println!("Running: {}", self.label);
        let mut app = App::new();

        app.insert_resource(WinitSettings {
            return_from_run: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(bevy::window::close_on_esc);

        (self.setup_graphics)(&mut app);
        (self.setup)(&mut app);
        app.run();
    }
}
