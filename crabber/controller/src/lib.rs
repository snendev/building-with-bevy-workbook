use bevy_app::{App, CoreSet, Plugin};
use bevy_ecs::{
    prelude::{Entity, Query, Without},
    query::Added,
    removal_detection::RemovedComponents,
    schedule::{IntoSystemConfigs, IntoSystemSetConfig, SystemSet},
    system::{Commands, ResMut},
};
use bevy_input::InputSystem as BevyInputSet;

use leafwing_input_manager::prelude::{
    ActionState, Actionlike, InputManagerBundle, InputManagerPlugin,
};

use crabber_core::EntityActionMap;
use crabber_protocol::{components::Knockout, inputs::InputAction};

pub mod components;
use components::{
    ArrowKeysControllerBundle, Controller, GamepadControllerBundle, WASDControllerBundle,
};

#[derive(Actionlike, Clone, Debug)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Menu,
}

pub type PlayerActionState = ActionState<Action>;

fn get_action(action: &PlayerActionState) -> Option<InputAction> {
    if action.pressed(Action::Up) {
        Some(InputAction::Up)
    } else if action.pressed(Action::Down) {
        Some(InputAction::Down)
    } else if action.pressed(Action::Left) {
        Some(InputAction::Left)
    } else if action.pressed(Action::Right) {
        Some(InputAction::Right)
    } else {
        None
    }
}

fn queue_inputs(
    mut player_query: Query<(Entity, &PlayerActionState), Without<Knockout>>,
    mut input_map: ResMut<EntityActionMap>,
) {
    for (entity, action_state) in player_query.iter_mut() {
        if let Some(input_action) = get_action(action_state) {
            input_map.0.insert(entity, input_action);
        }
    }
}

// Detects addition of `Controller` which assigns the appropriate controller

fn attach_controllers(
    mut commands: Commands,
    new_controllers_query: Query<(Entity, &Controller), Added<Controller>>,
) {
    for (entity, controller) in new_controllers_query.iter() {
        let mut entity_builder = commands.entity(entity);
        match controller {
            Controller::Keyboard(0) => {
                entity_builder.insert(WASDControllerBundle::new());
            }
            Controller::Keyboard(1) => {
                entity_builder.insert(ArrowKeysControllerBundle::new());
            }
            Controller::Gamepad(gamepad) => {
                entity_builder.insert(GamepadControllerBundle::new(gamepad));
            }
            _ => {}
        }
    }
}

// detects removal of Controllers to remove controls
fn cleanup_removed_controllers(
    mut commands: Commands,
    mut removed_controllers: RemovedComponents<Controller>,
) {
    for entity in removed_controllers.iter() {
        commands
            .entity(entity)
            .remove::<InputManagerBundle<Action>>();
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, SystemSet)]
pub struct InputSet;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Action>::default())
            .configure_set(InputSet.in_base_set(CoreSet::PreUpdate).after(BevyInputSet))
            .add_systems(
                (
                    cleanup_removed_controllers,
                    attach_controllers,
                    queue_inputs,
                )
                    .chain()
                    .in_set(InputSet),
            );
    }
}
