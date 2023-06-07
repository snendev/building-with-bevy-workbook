use bevy_ecs::{
    prelude::Entity,
    query::{With, Without},
    system::{Query, ResMut, Resource},
};
use bevy_utils::HashMap;

use crabber_protocol::{
    components::{Controlled, Knockout, Position, StepMotor},
    inputs::InputAction,
};

#[derive(Clone, Default, Debug, Resource)]
pub struct EntityActionMap(pub HashMap<Entity, InputAction>);

pub fn process_inputs(
    // Each player entity and the associated input action for this tick
    mut queued_inputs: ResMut<EntityActionMap>,
    mut player_query: Query<(&mut Position, &mut StepMotor), (Without<Knockout>, With<Controlled>)>,
) {
    for (entity, action) in queued_inputs.0.drain() {
        if let Ok((mut position, mut motor)) = player_query.get_mut(entity) {
            if !motor.is_running() {
                motor.start(&mut position, action.get_direction());
            }
        }
    }
}
