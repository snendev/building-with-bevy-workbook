use common_e2e::Test;
use crabber_app::{build, GraphicsPlugin as CrabGraphicsPlugin};

fn main() {
    Test {
        label: "Test full client".to_string(),
        setup: |app| {
            build(app);
        },
        setup_graphics: |app| {
            app.add_plugin(CrabGraphicsPlugin);
        },
        frames: 60,
        check: |_, _| true,
    }
    .run();
}
