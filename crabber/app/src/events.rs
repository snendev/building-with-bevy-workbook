use bevy::{
    prelude::{Commands, EventReader},
    sprite::SpriteSheetBundle,
};

use naia_bevy_client::{
    events::{InsertComponentEvents, MessageEvents},
    Client, CommandsExt,
};

use crabber_controller::components::Controller;
use crabber_protocol::{
    channels::PlayerAssignmentChannel,
    components::{Car, Controlled, Raft},
    messages::PlayerAssignmentMessage,
};

use crate::components::{PredictionOf, SourceOf};

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
                .insert((PredictionOf(entity), Controlled)) // this is the prediction entity
                .id();

            commands
                .entity(entity)
                .remove::<SpriteSheetBundle>() // no need to show graphics for this
                .insert((
                    // this is the original source entity
                    SourceOf(prediction_entity),
                    // attach controls to this one so tick system can easily retrieve them
                    Controller::keyboard(0),
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
                .insert((PredictionOf(entity), Controlled))
                .id();

            commands.entity(entity).insert(SourceOf(prediction_entity));
        }
        for entity in event.read::<Car>() {
            let prediction_entity = commands
                .entity(entity)
                .duplicate()
                .insert((PredictionOf(entity), Controlled))
                .id();

            commands.entity(entity).insert(SourceOf(prediction_entity));
        }
    }
}
