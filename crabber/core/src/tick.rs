use bevy_ecs::prelude::{Commands, Entity, Query, With, Without};
use bevy_log::info;

use crabber_protocol::{
    components::{
        Car, ConstantMotor, Controlled, Crab, Knockout, Level, LevelRow, Position, Raft, Score,
        StepMotor, TileRow,
    },
    constants::TILE_SIZE_F32,
};

pub fn tick_constant_motors(
    mut motor_query: Query<(&mut Position, &ConstantMotor), With<Controlled>>,
) {
    for (mut position, motor) in motor_query.iter_mut() {
        motor.drive_and_loop(&mut position);
    }
}

pub fn tick_step_motors(
    mut motor_query: Query<(&mut Position, &mut StepMotor), (Without<Knockout>, With<Controlled>)>,
) {
    for (mut position, mut motor) in motor_query.iter_mut() {
        motor.drive(&mut position);
        info!(
            "tick_motors: x: {}, y: {}, step: {:?}",
            *position.x, *position.y, *motor.step,
        );
    }
}

pub fn tick_score(
    mut player_query: Query<(&mut Score, &Position), (Without<Knockout>, With<Controlled>)>,
) {
    for (mut score, position) in player_query.iter_mut() {
        let current_tile_row = (*position.y / 64.) as u16;
        if current_tile_row > *score.value {
            *score.value = current_tile_row;
        }
    }
}

fn do_tiles_collide(position_a: &Position, position_b: &Position) -> bool {
    let dx = *position_a.x - *position_b.x;
    let dy = *position_a.y - *position_b.y;
    dx.abs() < TILE_SIZE_F32 && dy.abs() < TILE_SIZE_F32
}

pub fn tick_road_collisions(
    mut commands: Commands,
    level_query: Query<&Level>,
    player_query: Query<
        (Entity, &Position, &StepMotor),
        (With<Crab>, Without<Knockout>, With<Controlled>),
    >,
    car_query: Query<&Position, (With<Car>, Without<Crab>, With<Controlled>)>,
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
pub fn tick_river_collisions(
    mut commands: Commands,
    level_query: Query<&Level>,
    mut player_query: Query<
        (Entity, &mut Position, &StepMotor),
        (With<Crab>, Without<Knockout>, With<Controlled>),
    >,
    raft_query: Query<(&Position, &ConstantMotor), (With<Raft>, Without<Crab>, With<Controlled>)>,
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
