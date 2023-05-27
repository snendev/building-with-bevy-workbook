use bevy_ecs::{
    prelude::Entity,
    query::{ReadOnlyWorldQuery, With, Without},
    system::Query,
};

use crate::{
    components::{Crab, Knockout, Position, StepMotor},
    messages::InputAction,
};

pub fn process_input(action: Option<InputAction>, position: &mut Position, motor: &mut StepMotor) {
    if let Some(direction) = action.map(|action| action.get_direction()) {
        if !motor.is_running() {
            motor.start(position, direction);
        }
    }
}

pub fn process_inputs<RF: ReadOnlyWorldQuery>(
    // Each player entity and the associated input action for this tick
    inputs: Vec<(Entity, Option<InputAction>)>,
    player_query: &mut Query<(&mut Position, &mut StepMotor), (With<Crab>, Without<Knockout>, RF)>,
) {
    for (entity, action) in inputs.into_iter() {
        if let Ok((mut position, mut motor)) = player_query.get_mut(entity) {
            process_input(action, &mut position, &mut motor);
        }
    }
}
