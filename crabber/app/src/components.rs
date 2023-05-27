use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct PredictionOf(pub Entity);

#[derive(Component)]
pub struct SourceOf(pub Entity);
