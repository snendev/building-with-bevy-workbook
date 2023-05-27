use bevy::{
    asset::{AssetServer, Assets},
    prelude::{Entity, Handle, Resource, Vec2},
    sprite::TextureAtlas,
};

use bevy_asset_loader::asset_collection::AssetCollection;

use naia_bevy_client::CommandHistory;

use crabber_protocol::messages::InputAction;

#[derive(Resource, AssetCollection)]
pub struct SpriteSheetAssets {
    #[asset(texture_atlas(tile_size_x = 64., tile_size_y = 64., columns = 4, rows = 2))]
    #[asset(path = "spritesheets/crab.png")]
    pub crab: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 64., tile_size_y = 64., columns = 4, rows = 1))]
    #[asset(path = "spritesheets/level.png")]
    pub level: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 64., tile_size_y = 64., columns = 3, rows = 2))]
    #[asset(path = "spritesheets/car.png")]
    pub car: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 80., tile_size_y = 64., columns = 1, rows = 2))]
    #[asset(path = "spritesheets/raft.png")]
    pub raft: Handle<TextureAtlas>,
}

#[derive(Resource, Default)]
pub struct TickHistory(pub CommandHistory<Vec<(Entity, InputAction)>>);
