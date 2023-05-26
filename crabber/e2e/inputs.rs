use bevy::{
    prelude::{Commands, IntoSystemAppConfig, OnEnter, Query, Res},
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};

use common_e2e::Test;

use crabber::{
    components::{Crab, StepMotor, Transform},
    resources::SpriteSheetAssets,
    AppState, GraphicsPlugin as CrabGraphicsPlugin, InputPlugin, WASDControllerBundle,
};

fn tick_step_motors(mut motor_query: Query<(&mut Transform, &mut StepMotor)>) {
    for (mut transform, mut motor) in motor_query.iter_mut() {
        motor.drive(&mut transform);
    }
}

fn spawn_crab(mut commands: Commands, spritesheets: Res<SpriteSheetAssets>) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: spritesheets.crab.clone(),
            sprite: TextureAtlasSprite::new(0),
            ..Default::default()
        },
        StepMotor::new(),
        WASDControllerBundle::new(),
        Crab,
    ));
}

fn main() {
    Test {
        label: "Test inputs".to_string(),
        setup: |app| {
            app.add_state::<AppState>()
                .add_plugin(InputPlugin)
                .add_system(spawn_crab.in_schedule(OnEnter(AppState::InGame)))
                .add_system(tick_step_motors);
        },
        setup_graphics: |app| {
            app.add_plugin(CrabGraphicsPlugin);
        },
        frames: 60,
        check: |_, _| true,
    }
    .run();
}
