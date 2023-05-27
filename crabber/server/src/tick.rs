use bevy_ecs::{
    event::EventReader,
    prelude::Entity,
    query::{With, Without},
    system::{Commands, ParamSet, Query},
};

use naia_bevy_server::{events::TickEvent, Server};

use crabber_protocol::{
    channels::PlayerInputChannel,
    components::{
        Car, ConstantMotor, Controlled, Crab, Knockout, Level, Position, Raft, Score, StepMotor,
    },
    inputs,
    messages::InputMessage,
    tick,
};

pub fn tick_events(
    mut commands: Commands,
    mut server: Server,
    mut player_query_set: ParamSet<(
        Query<(&mut Position, &mut StepMotor), (With<Crab>, Without<Knockout>, With<Controlled>)>,
        Query<(&mut Score, &Position), (With<Crab>, Without<Knockout>, With<Controlled>)>,
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
    level_query: Query<&Level>,
    mut tick_reader: EventReader<TickEvent>,
) {
    let mut has_ticked = false;

    for TickEvent(server_tick) in tick_reader.iter() {
        has_ticked = true;

        let mut messages = server.receive_tick_buffer_messages(server_tick);
        let mut player_actions = Vec::new();
        for (_user_key, command) in messages.read::<PlayerInputChannel, InputMessage>() {
            let Some(entity) = command.entity.get(&server) else { continue };
            if let Some(action) = command.action {
                player_actions.push((entity, action));
            }
        }
        inputs::process_inputs(player_actions, &mut player_query_set.p0());
        tick::tick_step_motors(&mut player_query_set.p0());
        tick::tick_constant_motors(&mut objects_query_set.p0());
        tick::tick_score(&mut player_query_set.p1());
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

    if has_ticked {
        for (_, user_key, entity) in server.scope_checks() {
            server.user_scope(&user_key).include(&entity);
        }
    }
}
