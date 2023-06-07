use bevy_app::IntoSystemAppConfig;
use bevy_ecs::{
    prelude::{Commands, OnEnter},
    schedule::SystemSet,
};

use common_e2e::Test;

use crabber_core::{EntityActionMap, TickActions, TickPlugin};
use crabber_protocol::{
    components::{Car, ConstantMotor, Controlled, Direction, Position, Raft},
    constants::TILE_SIZE_F32,
};

use crabber_graphics::{AssetsState, GraphicsPlugin as CrabGraphicsPlugin};

fn noop() -> Vec<TickActions> {
    vec![(0, EntityActionMap::default())]
}

fn spawn_raft(mut commands: Commands) {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
struct TickSet;

fn main() {
    Test {
        label: "Test constant motors".to_string(),
        setup: |app| {
            app.add_plugin(TickPlugin::new(TickSet, noop))
                .add_system(spawn_raft.in_schedule(OnEnter(AssetsState::Ready)));
        },
        setup_graphics: |app| {
            app.add_plugin(CrabGraphicsPlugin);
        },
        frames: 60,
        check: |_, _| true,
    }
    .run();
}
