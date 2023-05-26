use bevy::{
    prelude::{
        App, Assets, BuildChildren, Commands, Component, IntoSystemAppConfig, OnEnter, Plugin, Res,
        SpatialBundle, Transform,
    },
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
};
use bevy_ecs_tilemap::{
    prelude::{
        fill_tilemap_rect, get_tilemap_center_transform, TilemapGridSize, TilemapId, TilemapPlugin,
        TilemapSize, TilemapTexture, TilemapTileSize, TilemapType,
    },
    tiles::{TilePos, TileStorage, TileTextureIndex},
    TilemapBundle,
};
use rand::Rng;

use crate::{
    components::{Car, ConstantMotor, Direction, Raft},
    constants::{
        BACKGROUND_Z, LEVEL_HEIGHT_F32, LEVEL_HEIGHT_I16, LEVEL_HEIGHT_U32, LEVEL_WIDTH_F32,
        LEVEL_WIDTH_I16, LEVEL_WIDTH_U32, LEVEL_Z, TILE_SIZE_F32, TILE_SIZE_I16,
    },
    resources::SpriteSheetAssets,
    AppState,
};

// Helpful conversion types for tile positions
pub struct TileColumn(pub i16);
pub struct TileRow(pub i16);

impl From<f32> for TileColumn {
    fn from(x: f32) -> TileColumn {
        TileColumn(((x - TILE_SIZE_F32 / 2.) / TILE_SIZE_F32 + LEVEL_WIDTH_F32 / 2.) as i16)
    }
}

impl From<TileColumn> for f32 {
    fn from(TileColumn(x): TileColumn) -> f32 {
        ((x - LEVEL_WIDTH_I16 / 2) * TILE_SIZE_I16 + TILE_SIZE_I16 / 2) as f32
    }
}

impl From<f32> for TileRow {
    fn from(y: f32) -> TileRow {
        TileRow(((y - TILE_SIZE_F32 / 2.) / TILE_SIZE_F32 + LEVEL_HEIGHT_F32 / 2.) as i16)
    }
}

impl From<TileRow> for f32 {
    fn from(TileRow(y): TileRow) -> f32 {
        ((y - LEVEL_HEIGHT_I16 / 2) * TILE_SIZE_I16 + TILE_SIZE_I16 / 2) as f32
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LevelRow {
    Grass,
    River,
    Road,
    Finish,
}

// Nothing super special here, just a slapdash way to generate random rows
impl LevelRow {
    fn get_random_next(&self) -> LevelRow {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(0..=9);
        match self {
            LevelRow::Grass => match r {
                0..=3 => LevelRow::Grass,
                4..=6 => LevelRow::Road,
                _ => LevelRow::River,
            },
            LevelRow::River => match r {
                0..=4 => LevelRow::River,
                5..=7 => LevelRow::Grass,
                _ => LevelRow::Road,
            },
            LevelRow::Road => match r {
                0..=5 => LevelRow::Road,
                6..=8 => LevelRow::Grass,
                _ => LevelRow::River,
            },
            LevelRow::Finish => LevelRow::Finish,
        }
    }
}

fn select_random_left_or_right() -> Direction {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..=1) {
        0 => Direction::Left,
        _ => Direction::Right,
    }
}

fn build_random_row(row_index: i16) -> Vec<(Transform, ConstantMotor)> {
    let mut rng = rand::thread_rng();
    let speed = rng.gen_range(1.0..6.0);
    let direction = select_random_left_or_right();
    let y = f32::from(TileRow(row_index));

    let mut vec = Vec::new();
    let mut current_x: i16 = 0;
    while current_x < LEVEL_WIDTH_I16 {
        let position = rng.gen_range(current_x..LEVEL_WIDTH_I16);

        // spawn a random-size clump
        let num_rafts = rng.gen_range(1i16..3);

        // space this clump from the next clump by at least 1
        current_x = position + num_rafts + rng.gen_range(1i16..3);

        for index in 0..num_rafts {
            let x = f32::from(TileColumn(position + index));
            vec.push((
                Transform::from_xyz(x as f32, y as f32, LEVEL_Z),
                ConstantMotor { speed, direction },
            ));
        }
    }

    vec
}

fn create_level_and_spawn_entities(
    commands: &mut Commands,
    spritesheets: &SpriteSheetAssets,
) -> Vec<LevelRow> {
    let mut rng = rand::thread_rng();
    let mut level = Vec::new();
    // The level should start with grass
    let mut level_row_kind = LevelRow::Grass;
    level.push(level_row_kind);
    // Then we should go up to the N-1 row from there
    for row_index in 1..(LEVEL_HEIGHT_I16 - 1) {
        level_row_kind = level_row_kind.get_random_next();
        level.push(level_row_kind);

        if LevelRow::Road == level_row_kind {
            for (transform, motor) in build_random_row(row_index).into_iter() {
                let random_color_offset = rng.gen_range(0..=2);
                let start_sprite_index = if Direction::Left == motor.direction {
                    3
                } else {
                    0
                };
                commands.spawn((
                    Car,
                    motor,
                    SpriteSheetBundle {
                        texture_atlas: spritesheets.car.clone(),
                        sprite: TextureAtlasSprite::new(start_sprite_index + random_color_offset),
                        transform,
                        ..Default::default()
                    },
                ));
            }
        }
        if LevelRow::River == level_row_kind {
            for (transform, motor) in build_random_row(row_index).into_iter() {
                let sprite_index = if Direction::Left == motor.direction {
                    1
                } else {
                    0
                };
                let x_offset = if Direction::Left == motor.direction {
                    8.
                } else {
                    -8.
                };
                commands
                    .spawn((Raft, motor, SpatialBundle::from_transform(transform)))
                    .with_children(|parent| {
                        parent.spawn(SpriteSheetBundle {
                            texture_atlas: spritesheets.raft.clone(),
                            sprite: TextureAtlasSprite::new(sprite_index),
                            // spawn the sprite a little offset to account for the "waves"
                            transform: Transform::from_xyz(x_offset, 0., 0.),
                            ..Default::default()
                        });
                    });
            }
        }
    }
    // Finally, add a finish line.
    level.push(LevelRow::Finish);
    level
}

fn spawn_level_tilemap(
    commands: &mut Commands,
    level: Vec<LevelRow>,
    texture_atlas: &TextureAtlas,
) {
    let map_entity = commands.spawn_empty().id();

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

    for (y, level_row) in level.iter().enumerate() {
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
            commands,
            &mut tile_storage,
        );
    }

    let grid_size: TilemapGridSize = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(map_entity).insert((
        TilemapBundle {
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
        },
        Level(level),
    ));
}

pub fn setup_level(
    mut commands: Commands,
    spritesheets: Res<SpriteSheetAssets>,
    atlas_assets: Res<Assets<TextureAtlas>>,
) {
    let level = create_level_and_spawn_entities(&mut commands, &spritesheets);
    let texture_atlas = atlas_assets.get(&spritesheets.level).unwrap();
    spawn_level_tilemap(&mut commands, level, &texture_atlas);
}

#[derive(Component, Debug)]
pub struct Level(pub Vec<LevelRow>);

impl Level {
    pub fn is_row_of_kind(&self, row: i16, target: LevelRow) -> bool {
        self.0
            .get(row as usize)
            .and_then(|row_kind| if *row_kind == target { Some(()) } else { None })
            .is_some()
    }
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .add_system(setup_level.in_schedule(OnEnter(AppState::InGame)));
    }
}
