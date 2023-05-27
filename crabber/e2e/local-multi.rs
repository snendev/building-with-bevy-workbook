use bevy::{
    prelude::{Commands, IntoSystemAppConfig, OnEnter, Res},
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};

use common_e2e::Test;

use crabber::{
    components::{Crab, StepMotor, Transform},
    constants::PLAYER_Z,
    resources::SpriteSheetAssets,
    AppState, ArrowKeysControllerBundle, CoreGameLoopPlugin, GraphicsPlugin as CrabGraphicsPlugin,
    InputPlugin, LevelPlugin, TileRow, WASDControllerBundle,
};

fn spawn_players(mut commands: Commands, spritesheets: Res<SpriteSheetAssets>) {
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
    commands.spawn((
        Crab,
        SpriteSheetBundle {
            texture_atlas: spritesheets.crab.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(0., f32::from(TileRow(0)), PLAYER_Z),
            ..Default::default()
        },
        ArrowKeysControllerBundle::new(),
        StepMotor::new(),
    ));
}

fn main() {
    Test {
        label: "Test local multiplayer".to_string(),
        setup: |app| {
            app.add_state::<AppState>()
                .add_plugin(InputPlugin)
                .add_plugin(CoreGameLoopPlugin)
                .add_plugin(LevelPlugin)
                .add_system(spawn_players.in_schedule(OnEnter(AppState::InGame)));
        },
        setup_graphics: |app| {
            app.add_plugin(CrabGraphicsPlugin);
        },
        frames: 60,
        check: |_, _| true,
    }
    .run();
}
