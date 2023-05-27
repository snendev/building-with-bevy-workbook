use bevy_app::{App, Plugin};
use bevy_ecs::{
    prelude::{Commands, Entity, IntoSystemConfigs, Query, With, Without},
    query::ReadOnlyWorldQuery,
};

use crate::{
    components::{
        Car, ConstantMotor, Crab, Knockout, Level, LevelRow, Position, Raft, Score, StepMotor,
        TileRow,
    },
    constants::TILE_SIZE_F32,
};

pub fn tick_constant_motors<RF: ReadOnlyWorldQuery>(
    motor_query: &mut Query<(&mut Position, &ConstantMotor), (Without<Crab>, RF)>,
) {
    for (mut position, motor) in motor_query.iter_mut() {
        motor.drive_and_loop(&mut position);
    }
}

pub fn tick_step_motors<RF: ReadOnlyWorldQuery>(
    motor_query: &mut Query<(&mut Position, &mut StepMotor), (With<Crab>, Without<Knockout>, RF)>,
) {
    for (mut position, mut motor) in motor_query.iter_mut() {
        motor.drive(&mut position);
    }
}

pub fn tick_score(
    player_query: &mut Query<(&mut Score, &Position), (With<Crab>, Without<Knockout>)>,
) {
    for (mut score, position) in player_query.iter_mut() {
        let current_tile_row = (*position.y / 64.) as u16;
        if current_tile_row > *score.value {
            *score.value = current_tile_row;
        }
    }
}

pub fn tick_score_system(
    mut player_query: Query<(&mut Score, &Position), (With<Crab>, Without<Knockout>)>,
) {
    tick_score(&mut player_query)
}

fn do_tiles_collide(position_a: &Position, position_b: &Position) -> bool {
    let dx = *position_a.x - *position_b.x;
    let dy = *position_a.y - *position_b.y;
    dx.abs() < TILE_SIZE_F32 && dy.abs() < TILE_SIZE_F32
}

pub fn tick_road_collisions<RF: ReadOnlyWorldQuery>(
    commands: &mut Commands,
    level_query: &Query<&Level>,
    player_query: &Query<(Entity, &Position, &StepMotor), (With<Crab>, Without<Knockout>, RF)>,
    car_query: &Query<&Position, (With<Car>, Without<Raft>, Without<Crab>, RF)>,
) {
    if let Ok(level) = level_query.get_single() {
        for (entity, position, motor) in player_query.iter() {
            let row = TileRow::from(*position.y);
            if !motor.is_running()
                && level.is_row_of_kind(row, LevelRow::Road)
                && car_query
                    .iter()
                    .any(|car_position| do_tiles_collide(position, car_position))
            {
                // knockout the player if any car collides with the player!
                commands.entity(entity).insert(Knockout);
            }
        }
    }
}

// check whether the character is in the river, or carried by a raft
pub fn tick_river_collisions<RF: ReadOnlyWorldQuery>(
    commands: &mut Commands,
    level_query: &Query<&Level>,
    player_query: &mut Query<
        (Entity, &mut Position, &StepMotor),
        (With<Crab>, Without<Knockout>, RF),
    >,
    raft_query: &Query<(&Position, &ConstantMotor), (With<Raft>, Without<Car>, Without<Crab>, RF)>,
) {
    if let Ok(level) = level_query.get_single() {
        for (entity, mut position, motor) in player_query.iter_mut() {
            let row = TileRow::from(*position.y);
            let mut should_crab_ko = false;

            // if player is on a river
            if !motor.is_running() && level.is_row_of_kind(row, LevelRow::River) {
                if let Some(raft_motor) = raft_query.iter().find_map(|(raft_position, motor)| {
                    if do_tiles_collide(&position, raft_position) {
                        Some(motor)
                    } else {
                        None
                    }
                }) {
                    // and also colliding on a raft, player will KO if they are driven offscreen
                    should_crab_ko = raft_motor.drive_offscreen(&mut position);
                } else {
                    // and not on a raft, player is KO
                    should_crab_ko = true;
                }
            }

            if should_crab_ko {
                // knockout the player!
                commands.entity(entity).insert(Knockout);
            }
        }
    }
}

fn tick_constant_motors_system(
    mut motor_query: Query<(&mut Position, &ConstantMotor), (Without<Crab>, ())>,
) {
    tick_constant_motors(&mut motor_query);
}

fn tick_step_motors_system(
    mut motor_query: Query<(&mut Position, &mut StepMotor), (With<Crab>, Without<Knockout>, ())>,
) {
    tick_step_motors(&mut motor_query);
}

fn tick_road_collisions_system(
    mut commands: Commands,
    level_query: Query<&Level>,
    player_query: Query<(Entity, &Position, &StepMotor), (With<Crab>, Without<Knockout>, ())>,
    car_query: Query<&Position, (With<Car>, Without<Raft>, Without<Crab>, ())>,
) {
    tick_road_collisions(&mut commands, &level_query, &player_query, &car_query);
}

fn tick_river_collisions_system(
    mut commands: Commands,
    level_query: Query<&Level>,
    mut player_query: Query<
        (Entity, &mut Position, &StepMotor),
        (With<Crab>, Without<Knockout>, ()),
    >,
    raft_query: Query<(&Position, &ConstantMotor), (With<Raft>, Without<Car>, Without<Crab>, ())>,
) {
    tick_river_collisions(&mut commands, &level_query, &mut player_query, &raft_query);
}

pub struct CoreGameLoopPlugin;

impl Plugin for CoreGameLoopPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                tick_step_motors_system,
                tick_constant_motors_system,
                tick_road_collisions_system,
                tick_river_collisions_system,
                tick_score_system,
            )
                .chain(),
        );
    }
}
