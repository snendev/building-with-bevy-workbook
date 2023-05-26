use bevy::{
    asset::{AssetServer, Assets},
    prelude::{Handle, Resource, Vec2},
    sprite::TextureAtlas,
};

use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(Resource, AssetCollection)]
pub struct SpriteSheetAssets {
    #[asset(texture_atlas(tile_size_x = 64., tile_size_y = 64., columns = 4, rows = 2))]
    #[asset(path = "spritesheets/crab.png")]
    pub crab: Handle<TextureAtlas>,
}
