use bevy::{
    prelude::{Commands, CoreSet, IntoSystemAppConfig, OnEnter, Res, SystemSet},
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};

use common_e2e::Test;

use crabber_graphics::{resources::SpriteSheetAssets, AssetsState, GraphicsPlugin as CrabGraphicsPlugin};

fn spawn_sprite(mut commands: Commands, spritesheets: Res<SpriteSheetAssets>) {
    commands.spawn(SpriteSheetBundle {
        texture_atlas: spritesheets.crab.clone(),
        sprite: TextureAtlasSprite::new(0),
        ..Default::default()
    });
}

fn main() {
    Test {
        label: "Test basic crab stuff".to_string(),
        setup: |app| {
            app.add_system(spawn_sprite.in_schedule(OnEnter(AssetsState::Ready)));
        },
        setup_graphics: |app| {
            app.add_plugin(CrabGraphicsPlugin::new(GraphicsSet));
        },
        frames: 60,
        check: |_, _| true,
    }
    .run();
}
