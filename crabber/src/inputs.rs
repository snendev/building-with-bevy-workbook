use bevy::{
    app::{App, Plugin},
    input::InputSystem,
    prelude::{Bundle, CoreSet, IntoSystemConfig, KeyCode, Query, Transform, Without},
};

use leafwing_input_manager::prelude::{
    ActionState, Actionlike, InputManagerBundle, InputManagerPlugin, InputMap,
};

use crate::components::{Direction, Knockout, StepMotor};

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

fn register_inputs(
    mut player_query: Query<
        (&mut Transform, &mut StepMotor, &PlayerActionState),
        Without<Knockout>,
    >,
) {
    for (mut transform, mut motor, action) in player_query.iter_mut() {
        let direction = if action.pressed(Action::Up) {
            Some(Direction::Up)
        } else if action.pressed(Action::Down) {
            Some(Direction::Down)
        } else if action.pressed(Action::Left) {
            Some(Direction::Left)
        } else if action.pressed(Action::Right) {
            Some(Direction::Right)
        } else {
            None
        };
        if let Some(direction) = direction {
            if !motor.is_running() {
                motor.start(&mut transform, direction);
            }
        }
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Action>::default())
            .add_system(
                register_inputs
                    .in_base_set(CoreSet::PreUpdate)
                    .after(InputSystem),
            );
    }
}
