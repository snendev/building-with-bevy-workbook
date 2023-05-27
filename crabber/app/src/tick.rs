use bevy::prelude::{Commands, Entity, EventReader, ParamSet, Query, ResMut, With, Without};

use naia_bevy_client::{events::ClientTickEvent, Client};

use crabber_protocol::{
    channels::PlayerInputChannel,
    components::{
        Car, ConstantMotor, Controlled, Crab, Knockout, Level, Position, Raft, StepMotor,
    },
    inputs::process_input,
    messages::InputMessage,
    tick,
};

use crate::{components::SourceOf, controller::QueuedInputs, resources::TickHistory};

pub fn tick(
    mut client: Client,
    mut commands: Commands,
    mut tick_reader: EventReader<ClientTickEvent>,
    level_query: Query<&Level>,
    source_query: Query<&SourceOf>,
    mut player_query_set: ParamSet<(
        Query<
            (Entity, &mut Position, &mut StepMotor),
            (With<Crab>, Without<Knockout>, With<Controlled>),
        >,
        Query<(&mut Position, &mut StepMotor), (With<Crab>, Without<Knockout>, With<Controlled>)>,
        Query<
            (Entity, &mut Position, &StepMotor),
            (With<Crab>, Without<Knockout>, With<Controlled>),
        >,
    )>,
    mut objects_query_set: ParamSet<(
        Query<(&mut Position, &ConstantMotor), (Without<Crab>, With<Controlled>)>,
        Query<&Position, (With<Car>, Without<Raft>, Without<Crab>, With<Controlled>)>,
        Query<
            (&Position, &ConstantMotor),
            (With<Raft>, Without<Car>, Without<Crab>, With<Controlled>),
        >,
    )>,
    mut player_inputs: ResMut<QueuedInputs>,
    mut tick_history: ResMut<TickHistory>,
) {
    let mut did_tick = false;
    for ClientTickEvent(client_tick) in tick_reader.iter() {
        did_tick = true;
        for (entity, action) in player_inputs.0.iter() {
            let Ok(SourceOf(prediction)) = source_query.get(*entity) else { continue };

            if let Ok((_, mut position, mut motor)) = player_query_set.p0().get_mut(*prediction) {
                let mut input_message = InputMessage::new(Some(*action));
                input_message.entity.set(&client, &entity);
                // Send command to server
                client.send_tick_buffer_message::<PlayerInputChannel, InputMessage>(
                    client_tick,
                    &input_message,
                );
                if let Some(action) = input_message.action {
                    process_input(action, &mut position, &mut motor);
                }
            }
        }

        // Store the commands in tick history for rollback
        let tick_record: Vec<(Entity, crabber_protocol::messages::InputAction)> = player_inputs
            .0
            .iter()
            .map(|(entity, action)| (*entity, *action))
            .collect::<Vec<_>>();
        tick_history.0.insert(*client_tick, tick_record);

        // Process the tick
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
    if did_tick {
        player_inputs.0.clear();
    }
}
