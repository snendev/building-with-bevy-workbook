use bevy_ecs::prelude::Component;

use naia_bevy_shared::Replicate;

#[derive(Component, Replicate)]
pub struct Crab;
#[derive(Component, Replicate)]
pub struct Raft;
#[derive(Component, Replicate)]
pub struct Car;

#[derive(Component, Replicate)]
pub struct Knockout;
