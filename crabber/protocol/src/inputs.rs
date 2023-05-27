use bevy_ecs::{
    prelude::Entity,
    query::{With, Without},
    system::Query,
};

use crate::{
    components::{Controlled, Crab, Knockout, Position, StepMotor},
    messages::InputAction,
};

pub fn process_input(action: InputAction, position: &mut Position, motor: &mut StepMotor) {
    if !motor.is_running() {
        motor.start(position, action.get_direction());
    }
}

pub fn process_inputs(
    // Each player entity and the associated input action for this tick
    inputs: Vec<(Entity, InputAction)>,
    player_query: &mut Query<
        (&mut Position, &mut StepMotor),
        (With<Crab>, Without<Knockout>, With<Controlled>),
    >,
) {
    for (entity, action) in inputs.into_iter() {
        if let Ok((mut position, mut motor)) = player_query.get_mut(entity) {
            process_input(action, &mut position, &mut motor);
        }
    }
}
