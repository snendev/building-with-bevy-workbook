use bevy_ecs::prelude::{Bundle, Component};
use bevy_input::prelude::{Gamepad, GamepadButtonType, KeyCode};

use leafwing_input_manager::prelude::{ActionState, InputManagerBundle, InputMap};

use crate::Action;

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

#[derive(Bundle)]
pub struct GamepadControllerBundle {
    input_manager: InputManagerBundle<Action>,
}

impl GamepadControllerBundle {
    pub fn new(gamepad: &Gamepad) -> Self {
        GamepadControllerBundle {
            input_manager: InputManagerBundle::<Action> {
                action_state: ActionState::default(),
                input_map: InputMap::default()
                    .set_gamepad(*gamepad)
                    .insert_multiple([
                        (GamepadButtonType::DPadUp, Action::Up),
                        (GamepadButtonType::DPadLeft, Action::Left),
                        (GamepadButtonType::DPadDown, Action::Down),
                        (GamepadButtonType::DPadRight, Action::Right),
                        (GamepadButtonType::Start, Action::Menu),
                    ])
                    .build(),
            },
        }
    }
}

// Crabber supports any number of gamepad controllers
// and two keyboard controllers, indexed 0 and 1, which
// are WASD and arrow keys, respectively.
#[derive(Component)]
pub enum Controller {
    Keyboard(usize),
    Gamepad(Gamepad),
}

impl Controller {
    pub fn keyboard(id: usize) -> Self {
        Controller::Keyboard(id)
    }

    pub fn gamepad(gamepad: Gamepad) -> Self {
        Controller::Gamepad(gamepad)
    }
}
