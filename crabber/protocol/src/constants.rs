pub const LEVEL_WIDTH_I16: i16 = 10;
pub const LEVEL_WIDTH_U32: u32 = 10;
pub const LEVEL_WIDTH_F32: f32 = 10.;

pub const LEVEL_HEIGHT_I16: i16 = 10;
pub const LEVEL_HEIGHT_U32: u32 = 10;
pub const LEVEL_HEIGHT_F32: f32 = 10.;

pub const TILE_SIZE_I16: i16 = 64;
pub const TILE_SIZE_F32: f32 = 64.;

pub const BACKGROUND_Z: f32 = 0.;
pub const LEVEL_Z: f32 = 3.;
pub const PLAYER_Z: f32 = 5.;

pub const MAX_X_I16: i16 = (LEVEL_WIDTH_I16 / 2 - 1) * TILE_SIZE_I16;
pub const MAX_X_F32: f32 = (LEVEL_WIDTH_F32 / 2. - 1.) * TILE_SIZE_F32;
pub const MAX_Y_I16: i16 = (LEVEL_HEIGHT_I16 / 2 - 1) * TILE_SIZE_I16;
pub const MAX_Y_F32: f32 = (LEVEL_HEIGHT_F32 / 2. - 1.) * TILE_SIZE_F32;
