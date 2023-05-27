use bevy::{
    prelude::{Commands, IntoSystemAppConfig, OnEnter, Res, Transform},
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};

use common_e2e::Test;

use crabber_protocol::{
    components::{Crab, StepMotor, TileRow},
    constants::PLAYER_Z,
    tick::CoreGameLoopPlugin,
};

use crabber_app::{
    resources::SpriteSheetAssets, AppState, ControllerPlugin, GraphicsPlugin as CrabGraphicsPlugin,
    WASDControllerBundle,
};

fn spawn_sprite(mut commands: Commands, spritesheets: Res<SpriteSheetAssets>) {
    commands.spawn((
        Crab,
        SpriteSheetBundle {
            texture_atlas: spritesheets.crab.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(0., f32::from(TileRow(0)), PLAYER_Z),
            ..Default::default()
        },
        WASDControllerBundle::new(),
        StepMotor::new(),
    ));
}

fn main() {
    Test {
        label: "Test common full game".to_string(),
        setup: |app| {
            app.add_state::<AppState>()
                .add_plugin(ControllerPlugin)
                .add_plugin(CoreGameLoopPlugin)
                .add_system(spawn_sprite.in_schedule(OnEnter(AppState::InGame)));
        },
        setup_graphics: |app| {
            app.add_plugin(CrabGraphicsPlugin);
        },
        frames: 60,
        check: |_, _| true,
    }
    .run();
}
