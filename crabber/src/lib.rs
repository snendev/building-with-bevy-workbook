use bevy::prelude::States;

mod graphics;
pub use graphics::GraphicsPlugin;

pub mod resources;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    InGame,
}
