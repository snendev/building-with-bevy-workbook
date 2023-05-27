use bevy::prelude::States;

mod graphics;
pub use graphics::GraphicsPlugin;
mod inputs;
pub use inputs::{Action, ArrowKeysControllerBundle, InputPlugin, WASDControllerBundle};
mod level;
pub use level::{LevelPlugin, TileColumn, TileRow};
mod tick;
pub use tick::CoreGameLoopPlugin;

pub mod components;
pub mod constants;
pub mod resources;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    InGame,
}
