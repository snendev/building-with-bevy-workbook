use bevy_ecs::component::Component;
use naia_bevy_shared::{Property, Replicate, Serde};
use rand::Rng;

use crate::{
    components::{Car, ConstantMotor, Direction, Position, Raft},
    constants::{
        LEVEL_HEIGHT_F32, LEVEL_HEIGHT_I16, LEVEL_WIDTH_F32, LEVEL_WIDTH_I16, TILE_SIZE_F32,
        TILE_SIZE_I16,
    },
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

#[derive(Clone, Copy, Debug, PartialEq, Serde)]
pub enum LevelRow {
    Grass,
    River,
    Road,
    Finish,
}

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

fn build_random_motors(row_index: i16) -> Vec<(Position, ConstantMotor)> {
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
                Position::new(x as f32, y as f32, direction),
                ConstantMotor::new(speed, direction),
            ));
        }
    }

    vec
}

#[derive(Component, Replicate)]
pub struct Level {
    pub rows: Property<Vec<LevelRow>>,
}

impl Level {
    pub fn new_random() -> Self {
        let mut rows = Vec::new();
        // The level should start with grass
        let mut level_row_kind = LevelRow::Grass;
        rows.push(level_row_kind);
        // Then we should go up to the N-1 row from there
        for _ in 1..(LEVEL_HEIGHT_I16 - 1) {
            level_row_kind = level_row_kind.get_random_next();
            rows.push(level_row_kind);
        }
        // Finally, add a finish line.
        rows.push(LevelRow::Finish);
        Level::new_complete(rows)
    }

    pub fn create_level_bundles(
        &self,
    ) -> (
        Vec<(Car, Position, ConstantMotor)>,
        Vec<(Raft, Position, ConstantMotor)>,
    ) {
        let mut car_bundles = Vec::new();
        let mut raft_bundles = Vec::new();
        // Then we should go up to the N-1 row from there
        for (row_index, row_kind) in self.rows.iter().enumerate() {
            if LevelRow::Road == *row_kind {
                for (position, motor) in build_random_motors(row_index as i16).into_iter() {
                    car_bundles.push((Car, position, motor));
                }
            }
            if LevelRow::River == *row_kind {
                for (position, motor) in build_random_motors(row_index as i16).into_iter() {
                    raft_bundles.push((Raft, position, motor));
                }
            }
        }
        (car_bundles, raft_bundles)
    }

    pub fn is_row_of_kind(&self, row: TileRow, target: LevelRow) -> bool {
        self.rows
            .get(row.0 as usize)
            .and_then(|row_kind| if *row_kind == target { Some(()) } else { None })
            .is_some()
    }
}
