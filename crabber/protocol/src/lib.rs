use std::time::Duration;

use naia_bevy_shared::{LinkConditionerConfig, Protocol, ProtocolPlugin};

pub mod bundles;
pub mod channels;
pub mod components;
pub mod constants;
pub mod inputs;
pub mod messages;
pub mod tick;

struct CrabberProtocolPlugin;

impl ProtocolPlugin for CrabberProtocolPlugin {
    fn build(&self, protocol: &mut Protocol) {
        channels::PlayerInputChannel::add_to_protocol(protocol);
        channels::PlayerAssignmentChannel::add_to_protocol(protocol);

        protocol
            .add_message::<messages::PlayerAssignmentMessage>()
            .add_message::<messages::InputMessage>()
            .add_component::<components::Crab>()
            .add_component::<components::Car>()
            .add_component::<components::Raft>()
            .add_component::<components::Position>()
            .add_component::<components::ConstantMotor>()
            .add_component::<components::StepMotor>()
            .add_component::<components::Knockout>()
            .add_component::<components::Level>()
            .add_component::<components::Score>();
    }
}

pub fn protocol() -> Protocol {
    Protocol::builder()
        .tick_interval(Duration::from_millis(16))
        .link_condition(LinkConditionerConfig::good_condition())
        .add_plugin(CrabberProtocolPlugin)
        .build()
}
