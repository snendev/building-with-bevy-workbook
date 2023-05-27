use bevy::{
    app::{App, Plugin},
    input::InputSystem,
    prelude::{
        Bundle, CoreSet, Entity, IntoSystemConfig, KeyCode, Query, ResMut, Resource, Without,
    },
    utils::HashMap,
};

use leafwing_input_manager::prelude::{
    ActionState, Actionlike, InputManagerBundle, InputManagerPlugin, InputMap,
};

use crabber_protocol::{components::Knockout, messages::InputAction};

#[derive(Actionlike, Clone, Debug)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Menu,
}

pub type PlayerActionState = ActionState<Action>;

#[derive(Bundle)]
pub struct WASDControllerBundle {
    input_manager: InputManagerBundle<Action>,
}

impl WASDControllerBundle {
    pub fn new() -> Self {
        WASDControllerBundle {
            input_manager: InputManagerBundle::<Action> {
                action_state: ActionState::default(),
                input_map: InputMap::new([
                    (KeyCode::W, Action::Up),
                    (KeyCode::A, Action::Left),
                    (KeyCode::S, Action::Down),
                    (KeyCode::D, Action::Right),
                    (KeyCode::Escape, Action::Menu),
                ])
                .build(),
            },
        }
    }
}

#[derive(Bundle)]
pub struct ArrowKeysControllerBundle {
    input_manager: InputManagerBundle<Action>,
}

impl ArrowKeysControllerBundle {
    pub fn new() -> Self {
        ArrowKeysControllerBundle {
            input_manager: InputManagerBundle::<Action> {
                action_state: ActionState::default(),
                input_map: InputMap::new([
                    (KeyCode::Up, Action::Up),
                    (KeyCode::Left, Action::Left),
                    (KeyCode::Down, Action::Down),
                    (KeyCode::Right, Action::Right),
                    (KeyCode::Escape, Action::Menu),
                ])
                .build(),
            },
        }
    }
}

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

#[derive(Default, Resource)]
pub struct QueuedInputs(pub HashMap<Entity, InputAction>);

fn register_inputs(
    mut player_query: Query<(Entity, &PlayerActionState), Without<Knockout>>,
    mut player_inputs: ResMut<QueuedInputs>,
) {
    for (entity, action_state) in player_query.iter_mut() {
        if let Some(input_action) = get_action(action_state) {
            player_inputs.0.insert(entity, input_action);
        }
    }
}

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Action>::default())
            .init_resource::<QueuedInputs>()
            .add_system(
                register_inputs
                    .in_base_set(CoreSet::PreUpdate)
                    .after(InputSystem),
            );
    }
}
