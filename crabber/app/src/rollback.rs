use bevy::prelude::{Entity, EventReader, Query, ResMut, With, Without};

use crabber_core::TickActions;
use naia_bevy_client::{events::UpdateComponentEvents, sequence_greater_than, Replicate, Tick};

use crabber_protocol::components::{Controlled, Crab, Knockout, Position, StepMotor};

use crate::{components::SourceOf, resources::TickHistory};

pub fn receive_update_component_events(
    mut event_reader: EventReader<UpdateComponentEvents>,
    mut tick_history: ResMut<TickHistory>,
    source_player_query: Query<
        (&Position, &StepMotor, &SourceOf),
        (With<Crab>, Without<Controlled>),
    >,
    source_objects_query: Query<(&Position, &SourceOf), (Without<Crab>, Without<Controlled>)>,
    mut player_query: Query<
        (Entity, &mut Position, &mut StepMotor),
        (With<Crab>, Without<Knockout>, With<Controlled>),
    >,
    mut objects_query: Query<&mut Position, (Without<Crab>, With<Controlled>)>,
) -> Vec<TickActions> {
    // We only care about whatever the latest tick is
    // so we check the events for the latest tick count,
    // and use that to get the commands we need to replay
    let mut latest_tick: Option<Tick> = None;
    for events in event_reader.iter() {
        for (server_tick, _entity) in events.read::<Position>() {
            if let Some(last_tick) = latest_tick {
                if sequence_greater_than(server_tick, last_tick) {
                    latest_tick = Some(server_tick);
                }
            } else {
                latest_tick = Some(server_tick);
            }
        }
    }
    if let Some(latest_tick) = latest_tick {
        // Reset all expected entities to their source states
        for (source_position, source_motor, SourceOf(prediction)) in source_player_query.iter() {
            if let Ok((_, mut position, mut motor)) = player_query.get_mut(*prediction) {
                position.mirror(source_position);
                motor.mirror(source_motor);
            }
        }
        for (source_position, SourceOf(prediction)) in source_objects_query.iter() {
            if let Ok(mut position) = objects_query.get_mut(*prediction) {
                position.mirror(source_position);
            }
        }
        // Then replay ticks
        let mut replays = tick_history.0.replays(&latest_tick);
        replays.reverse();
        replays
    } else {
        Vec::new()
    }
}
