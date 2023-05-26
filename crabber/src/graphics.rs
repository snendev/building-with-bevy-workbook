use bevy::prelude::{App, Camera2dBundle, Commands, IntoSystemAppConfig, OnEnter, Plugin};

use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};

use crate::{resources::SpriteSheetAssets, AppState};

fn camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading).continue_to_state(AppState::InGame),
        )
        .add_collection_to_loading_state::<_, SpriteSheetAssets>(AppState::Loading)
        .add_system(camera.in_schedule(OnEnter(AppState::InGame)));
    }
}
