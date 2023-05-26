use bevy::{
    prelude::{Commands, IntoSystemAppConfig, OnEnter, Res},
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};

use common_e2e::Test;

use crabber::{
    components::{ConstantMotor, Direction},
    resources::SpriteSheetAssets,
    AppState, CoreGameLoopPlugin, GraphicsPlugin as CrabGraphicsPlugin,
};

fn spawn_raft(mut commands: Commands, spritesheets: Res<SpriteSheetAssets>) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: spritesheets.car.clone(),
            sprite: TextureAtlasSprite::new(0),
            ..Default::default()
        },
        ConstantMotor {
            speed: 4.,
            direction: Direction::Right,
        },
    ));
}

fn main() {
    Test {
        label: "Test constant motors".to_string(),
        setup: |app| {
            app.add_state::<AppState>()
                .add_plugin(CoreGameLoopPlugin)
                .add_system(spawn_raft.in_schedule(OnEnter(AppState::InGame)));
        },
        setup_graphics: |app| {
            app.add_plugin(CrabGraphicsPlugin);
        },
        frames: 60,
        check: |_, _| true,
    }
    .run();
}
