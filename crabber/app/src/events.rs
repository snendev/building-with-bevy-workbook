use bevy::{
    prelude::{Commands, Entity, EventReader, ParamSet, Query, ResMut, With, Without},
    sprite::SpriteSheetBundle,
};

use crabber_protocol::{
    channels::PlayerAssignmentChannel,
    components::{Car, ConstantMotor, Crab, Knockout, Level, Position, Raft, StepMotor},
    inputs::process_input,
    messages::PlayerAssignmentMessage,
    tick,
};
use naia_bevy_client::{
    events::{InsertComponentEvents, MessageEvents, UpdateComponentEvents},
    sequence_greater_than, Client, CommandsExt, Replicate, Tick,
};

use crate::{
    components::{PredictionOf, SourceOf},
    resources::TickHistory,
    WASDControllerBundle,
};

pub fn receive_entity_assignment_message(
    mut event_reader: EventReader<MessageEvents>,
    mut commands: Commands,
    client: Client,
) {
    for event in event_reader.iter() {
        for assignment in event.read::<PlayerAssignmentChannel, PlayerAssignmentMessage>() {
            let entity = assignment.entity.get(&client).unwrap();
            let prediction_entity = commands
                .entity(entity)
                .duplicate() // create a new entity and copy all `Replicate`
                .insert(PredictionOf(entity)) // this is the prediction entity
                .id();

            commands
                .entity(entity)
                .remove::<SpriteSheetBundle>() // no need to show graphics for this
                .insert((
                    // this is the original source entity
                    SourceOf(prediction_entity),
                    // attach controls to this one so tick system can easily retrieve them
                    WASDControllerBundle::new(),
                ));
        }
    }
}

pub fn receive_insert_component_events(
    mut commands: Commands,
    mut event_reader: EventReader<InsertComponentEvents>,
) {
    for event in event_reader.iter() {
        for entity in event.read::<Raft>() {
            let prediction_entity = commands
                .entity(entity)
                .duplicate()
                .insert(PredictionOf(entity))
                .id();

            commands.entity(entity).insert(SourceOf(prediction_entity));
        }
        for entity in event.read::<Car>() {
            let prediction_entity = commands
                .entity(entity)
                .duplicate()
                .insert(PredictionOf(entity))
                .id();

            commands.entity(entity).insert(SourceOf(prediction_entity));
        }
    }
}

pub fn receive_update_component_events(
    mut commands: Commands,
    mut event_reader: EventReader<UpdateComponentEvents>,
    mut tick_history: ResMut<TickHistory>,
    source_player_query: Query<
        (&Position, &StepMotor, &SourceOf),
        (With<Crab>, Without<PredictionOf>),
    >,
    source_objects_query: Query<(&Position, &SourceOf), (Without<Crab>, Without<PredictionOf>)>,
    mut player_query_set: ParamSet<(
        Query<
            (Entity, &mut Position, &mut StepMotor),
            (With<Crab>, Without<Knockout>, With<PredictionOf>),
        >,
        Query<(&mut Position, &mut StepMotor), (With<Crab>, Without<Knockout>, With<PredictionOf>)>,
        Query<
            (Entity, &mut Position, &StepMotor),
            (With<Crab>, Without<Knockout>, With<PredictionOf>),
        >,
    )>,
    mut objects_query_set: ParamSet<(
        Query<(&mut Position, &ConstantMotor), (Without<Crab>, With<PredictionOf>)>,
        Query<&Position, (With<Car>, Without<Raft>, Without<Crab>, With<PredictionOf>)>,
        Query<
            (&Position, &ConstantMotor),
            (With<Raft>, Without<Car>, Without<Crab>, With<PredictionOf>),
        >,
    )>,
    level_query: Query<&Level>,
) {
    for events in event_reader.iter() {
        // We only care about whatever the latest tick is
        // so we check the events for the latest tick count,
        // and use that to get the commands we need to replay
        let mut latest_tick: Option<Tick> = None;

        for (server_tick, _entity) in events.read::<Position>() {
            if let Some(last_tick) = latest_tick {
                if sequence_greater_than(server_tick, last_tick) {
                    latest_tick = Some(server_tick);
                }
            } else {
                latest_tick = Some(server_tick);
            }
        }

        if let Some(latest_tick) = latest_tick {
            // Reset all expected entities to their source states
            for (source_position, source_motor, SourceOf(prediction)) in source_player_query.iter()
            {
                if let Ok((_, mut position, mut motor)) = player_query_set.p0().get_mut(*prediction)
                {
                    position.mirror(source_position);
                    motor.mirror(source_motor);
                }
            }
            for (source_position, SourceOf(prediction)) in source_objects_query.iter() {
                if let Ok((mut position, _)) = objects_query_set.p0().get_mut(*prediction) {
                    position.mirror(source_position);
                }
            }
            // Then replay ticks
            let replay_ticks = tick_history.0.replays(&latest_tick);
            for (_tick, tick_actions) in replay_ticks.into_iter() {
                for (entity, action) in tick_actions.into_iter() {
                    if let Ok((_entity, mut position, mut motor)) =
                        player_query_set.p0().get_mut(entity)
                    {
                        process_input(Some(action), &mut position, &mut motor);
                    }
                    tick::tick_step_motors(&mut player_query_set.p1());
                    tick::tick_constant_motors(&mut objects_query_set.p0());
                    tick::tick_road_collisions(
                        &mut commands,
                        &level_query,
                        &player_query_set.p2().to_readonly(),
                        &objects_query_set.p1(),
                    );
                    tick::tick_river_collisions(
                        &mut commands,
                        &level_query,
                        &mut player_query_set.p2(),
                        &objects_query_set.p2(),
                    );
                }
            }
        }
    }
}
