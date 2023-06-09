use bevy::{
    prelude::{
        in_state, info, Added, App, Assets, BuildChildren, Camera2dBundle, Changed, Color,
        Commands, Component, Entity, IntoSystemConfigs, IntoSystemSetConfig, Plugin, Quat, Query,
        Res, SpatialBundle, States, SystemSet, Transform, With,
    },
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
};

use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};

use bevy_ecs_tilemap::{
    prelude::{
        fill_tilemap_rect, get_tilemap_center_transform, TilemapGridSize, TilemapId, TilemapPlugin,
        TilemapSize, TilemapTexture, TilemapTileSize, TilemapType,
    },
    tiles::{TilePos, TileStorage, TileTextureIndex},
    TilemapBundle,
};

use rand::Rng;

use crabber_protocol::{
    components::{
        Car, Controlled, Crab, Direction, Knockout, Level, LevelRow, Position, Raft, StepMotor,
    },
    constants::{
        BACKGROUND_Z, LEVEL_HEIGHT_U32, LEVEL_WIDTH_U32, LEVEL_Z, PLAYER_Z, TILE_SIZE_F32,
    },
};

mod resources;
use resources::SpriteSheetAssets;

#[derive(Component)]
struct ShouldRender;

fn direction_to_angle(direction: Direction) -> f32 {
    match direction {
        Direction::Up => 0.,
        Direction::Right => -std::f32::consts::FRAC_PI_2,
        Direction::Down => std::f32::consts::PI,
        Direction::Left => std::f32::consts::FRAC_PI_2,
    }
}

fn position_to_transform(position: &Position, z: f32, rotated: bool) -> Transform {
    if rotated {
        Transform::from_xyz(*position.x, *position.y, z).with_rotation(Quat::from_rotation_z(
            direction_to_angle(*position.direction),
        ))
    } else {
        Transform::from_xyz(*position.x, *position.y, z)
    }
}

fn sync_transforms(mut position_query: Query<(&Position, &mut Transform, Option<&Crab>)>) {
    for (position, mut transform, crab) in position_query.iter_mut() {
        *transform = position_to_transform(position, transform.translation.z, crab.is_some());
    }
}

fn handle_knockout(mut ko_query: Query<&mut TextureAtlasSprite, Added<Knockout>>) {
    for mut sprite in ko_query.iter_mut() {
        sprite.color = Color::rgba(1., 1., 1., 0.5);
        sprite.flip_y = true;
    }
}

fn setup_crab_sprites(
    mut commands: Commands,
    added_crabs_query: Query<(Entity, &Position, Option<&Controlled>), Added<Crab>>,
    spritesheets: Res<SpriteSheetAssets>,
) {
    for (entity, position, is_controlled) in added_crabs_query.iter() {
        let mut sprite = TextureAtlasSprite::new(0);
        if is_controlled.is_none() {
            // add a tint for non-player crabs
            sprite.color = Color::rgba(1., 1., 1., 0.75);
        }
        commands.entity(entity).insert((SpriteSheetBundle {
            texture_atlas: spritesheets.crab.clone(),
            sprite,
            transform: position_to_transform(position, PLAYER_Z, true),
            ..Default::default()
        },));
    }
}

fn get_car_sprite(direction: Direction) -> TextureAtlasSprite {
    let mut rng = rand::thread_rng();
    let random_color_offset = rng.gen_range(0..=2);
    let start_sprite_index = if direction == Direction::Left { 3 } else { 0 };
    TextureAtlasSprite::new(start_sprite_index + random_color_offset)
}

fn setup_car_sprites(
    mut commands: Commands,
    added_cars_query: Query<(Entity, &Position), (Added<Car>, With<Controlled>)>,
    spritesheets: Res<SpriteSheetAssets>,
) {
    for (entity, position) in added_cars_query.iter() {
        commands.entity(entity).insert(SpriteSheetBundle {
            texture_atlas: spritesheets.car.clone(),
            sprite: get_car_sprite(*position.direction),
            transform: position_to_transform(position, LEVEL_Z, false),
            ..Default::default()
        });
    }
}

fn setup_raft_sprites(
    mut commands: Commands,
    added_rafts_query: Query<(Entity, &Position), (Added<Raft>, With<Controlled>)>,
    spritesheets: Res<SpriteSheetAssets>,
) {
    for (entity, position) in added_rafts_query.iter() {
        let is_left = Direction::Left == *position.direction;
        let sprite_index = if is_left { 1 } else { 0 };
        let x_offset = if is_left { 8. } else { -8. };
        let transform = position_to_transform(position, LEVEL_Z, false);
        commands
            .entity(entity)
            .insert(SpatialBundle::from_transform(transform))
            .with_children(|parent| {
                parent.spawn(SpriteSheetBundle {
                    texture_atlas: spritesheets.raft.clone(),
                    sprite: TextureAtlasSprite::new(sprite_index),
                    // spawn the sprite with a small offset to account for the "waves"
                    transform: Transform::from_xyz(x_offset, 0., 0.),
                    ..Default::default()
                });
            });
    }
}

fn setup_level_tilemap(
    mut commands: Commands,
    level_query: Query<(Entity, &Level), Added<Level>>,
    spritesheets: Res<SpriteSheetAssets>,
    atlas_assets: Res<Assets<TextureAtlas>>,
) {
    if let Ok((map_entity, level)) = level_query.get_single() {
        info!("{:?}", map_entity);

        let tilemap_size = TilemapSize {
            x: LEVEL_WIDTH_U32,
            y: LEVEL_HEIGHT_U32,
        };
        let row_size = TilemapSize {
            x: LEVEL_WIDTH_U32,
            y: 1,
        };
        let tile_size = TilemapTileSize {
            x: TILE_SIZE_F32,
            y: TILE_SIZE_F32,
        };

        let mut tile_storage = TileStorage::empty(tilemap_size);

        for (y, level_row) in level.rows.iter().enumerate() {
            let texture_index = TileTextureIndex(match level_row {
                LevelRow::Grass => 1,
                LevelRow::River => 0,
                LevelRow::Road => 2,
                LevelRow::Finish => 3,
            });

            fill_tilemap_rect(
                texture_index,
                TilePos { x: 0, y: y as u32 },
                row_size,
                TilemapId(map_entity),
                &mut commands,
                &mut tile_storage,
            );
        }

        let grid_size: TilemapGridSize = tile_size.into();
        let map_type = TilemapType::default();
        let texture_atlas = atlas_assets.get(&spritesheets.level).unwrap();

        commands.entity(map_entity).insert(TilemapBundle {
            map_type,
            tile_size,
            grid_size,
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_atlas.texture.clone()),
            transform: get_tilemap_center_transform(
                &tilemap_size,
                &grid_size,
                &map_type,
                BACKGROUND_Z,
            ),
            ..Default::default()
        });
    }
}

fn animate_sprites(
    mut crab_query: Query<(&StepMotor, &mut TextureAtlasSprite), (Changed<StepMotor>, With<Crab>)>,
) {
    for (motor, mut sprite) in crab_query.iter_mut() {
        sprite.index = motor.get_sprite_index();
    }
}

fn camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, States)]
pub enum AssetsState {
    #[default]
    Loading, // assets are not ready
    Ready, // assets are ready
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, SystemSet)]
pub struct GraphicsSet;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .add_state::<AssetsState>()
            .configure_set(GraphicsSet.run_if(in_state(AssetsState::Ready)))
            .add_loading_state(
                LoadingState::new(AssetsState::Loading).continue_to_state(AssetsState::Ready),
            )
            .add_collection_to_loading_state::<_, SpriteSheetAssets>(AssetsState::Loading)
            .add_startup_system(camera)
            .add_systems(
                (
                    handle_knockout,
                    setup_crab_sprites,
                    setup_car_sprites,
                    setup_raft_sprites,
                    setup_level_tilemap,
                    animate_sprites,
                    sync_transforms,
                )
                    .in_set(GraphicsSet),
            );
    }
}
