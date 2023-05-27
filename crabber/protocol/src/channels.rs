use naia_bevy_shared::{
    Channel, ChannelDirection, ChannelMode, Protocol, ReliableSettings, TickBufferSettings,
};

#[derive(Channel)]
pub struct PlayerInputChannel;
impl PlayerInputChannel {
    pub fn add_to_protocol(protocol: &mut Protocol) {
        protocol.add_channel::<PlayerInputChannel>(
            ChannelDirection::ClientToServer,
            ChannelMode::TickBuffered(TickBufferSettings::default()),
        );
    }
}

#[derive(Channel)]
pub struct PlayerAssignmentChannel;

impl PlayerAssignmentChannel {
    pub fn add_to_protocol(protocol: &mut Protocol) {
        protocol.add_channel::<PlayerAssignmentChannel>(
            ChannelDirection::ServerToClient,
            ChannelMode::UnorderedReliable(ReliableSettings::default()),
        );
    }
}
