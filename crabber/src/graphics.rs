use bevy::prelude::{
    App, Camera2dBundle, Changed, Commands, CoreSet, IntoSystemAppConfig, IntoSystemConfig,
    OnEnter, Plugin, Query, TextureAtlasSprite, With,
};

use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};

use crate::{
    components::{Crab, StepMotor},
    resources::SpriteSheetAssets,
    AppState,
};

fn camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn animate_sprites(
    mut crab_query: Query<(&StepMotor, &mut TextureAtlasSprite), (Changed<StepMotor>, With<Crab>)>,
) {
    for (motor, mut sprite) in crab_query.iter_mut() {
        sprite.index = motor.get_sprite_index();
    }
}

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading).continue_to_state(AppState::InGame),
        )
        .add_collection_to_loading_state::<_, SpriteSheetAssets>(AppState::Loading)
        .add_system(camera.in_schedule(OnEnter(AppState::InGame)))
        .add_system(animate_sprites.in_base_set(CoreSet::PostUpdate));
    }
}
