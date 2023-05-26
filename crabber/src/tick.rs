use bevy::{
    app::{App, Plugin},
    prelude::{Added, Color, Commands, Entity, IntoSystemConfigs, Query, With, Without},
    sprite::TextureAtlasSprite,
};

use crate::{
    components::{Car, ConstantMotor, Crab, Knockout, Raft, Score, StepMotor, Transform},
    constants::TILE_SIZE_F32,
    level::{Level, LevelRow, TileRow},
};

fn tick_constant_motors(mut motor_query: Query<(&mut Transform, &ConstantMotor)>) {
    for (mut transform, motor) in motor_query.iter_mut() {
        motor.drive_and_loop(&mut transform);
    }
}

fn tick_step_motors(mut motor_query: Query<(&mut Transform, &mut StepMotor), Without<Knockout>>) {
    for (mut transform, mut motor) in motor_query.iter_mut() {
        motor.drive(&mut transform);
    }
}

fn tick_score(mut player_query: Query<(&mut Score, &Transform), Without<Knockout>>) {
    for (mut score, transform) in player_query.iter_mut() {
        let current_tile_row = (transform.translation.y / 64.) as u16;
        if current_tile_row > score.0 {
            score.0 = current_tile_row;
        }
    }
}

fn do_tiles_collide(transform_a: &Transform, transform_b: &Transform) -> bool {
    let dx = transform_a.translation.x - transform_b.translation.x;
    let dy = transform_a.translation.y - transform_b.translation.y;
    dx.abs() < TILE_SIZE_F32 && dy.abs() < TILE_SIZE_F32
}

fn tick_road_collisions(
    mut commands: Commands,
    level_query: Query<&Level>,
    player_query: Query<(Entity, &Transform, &StepMotor), With<Crab>>,
    car_query: Query<(&Car, &Transform)>,
) {
    if let Ok(level) = level_query.get_single() {
        for (entity, transform, motor) in player_query.iter() {
            let TileRow(row) = TileRow::from(transform.translation.y);
            if !motor.is_running()
                && level.is_row_of_kind(row, LevelRow::Road)
                && car_query
                    .iter()
                    .any(|(_, car_transform)| do_tiles_collide(transform, car_transform))
            {
                // knockout the player if any car collides with the player!
                commands.entity(entity).insert(Knockout);
            }
        }
    }
}

// check whether the character is in the river, or carried by a raft
fn tick_river_collisions(
    mut commands: Commands,
    level_query: Query<&Level>,
    mut player_query: Query<(Entity, &mut Transform, &StepMotor), (With<Crab>, Without<Knockout>)>,
    raft_query: Query<(&Transform, &ConstantMotor), (With<Raft>, Without<Crab>)>,
) {
    if let Ok(level) = level_query.get_single() {
        for (entity, mut transform, motor) in player_query.iter_mut() {
            let TileRow(row) = TileRow::from(transform.translation.y);
            let mut should_crab_ko = false;

            // if player is on a river
            if !motor.is_running() && level.is_row_of_kind(row, LevelRow::River) {
                if let Some(raft_motor) = raft_query.iter().find_map(|(raft_transform, motor)| {
                    if do_tiles_collide(&transform, raft_transform) {
                        Some(motor)
                    } else {
                        None
                    }
                }) {
                    // and also colliding on a raft, player will KO if they are driven offscreen
                    should_crab_ko = raft_motor.drive_offscreen(&mut transform);
                } else {
                    // and not on a raft, player is KO
                    should_crab_ko = true;
                }
            }

            if should_crab_ko {
                commands.entity(entity).insert(Knockout);
            }
        }
    }
}

fn handle_knockout(mut ko_query: Query<&mut TextureAtlasSprite, Added<Knockout>>) {
    for mut sprite in ko_query.iter_mut() {
        sprite.color = Color::rgba(1., 1., 1., 0.5);
        sprite.flip_y = true;
    }
}

pub struct CoreGameLoopPlugin;

impl Plugin for CoreGameLoopPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                tick_step_motors,
                tick_river_collisions,
                tick_road_collisions,
                tick_constant_motors,
                tick_score,
            )
                .chain(),
        )
        .add_system(handle_knockout);
    }
}
