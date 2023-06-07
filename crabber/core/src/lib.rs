use std::marker::PhantomData;

use bevy_app::{App, Plugin};
use bevy_ecs::{
    schedule::{
        FreeSystemSet, IntoSystemConfig, IntoSystemConfigs, Schedule, ScheduleLabel, SystemSet,
    },
    system::{In, IntoPipeSystem, IntoSystem},
    world::World,
};

mod inputs;
pub use inputs::EntityActionMap;

mod tick;

#[derive(Debug, Hash, PartialEq, Eq, Clone, ScheduleLabel)]
pub struct CoreTickSchedule;

fn build_core_tick_schedule() -> Schedule {
    let mut schedule = Schedule::new();
    schedule
        .add_systems(
            (
                inputs::process_inputs,
                tick::tick_constant_motors,
                tick::tick_step_motors,
            )
                .chain(),
        )
        .add_systems(
            (
                tick::tick_river_collisions,
                tick::tick_road_collisions,
                tick::tick_score,
            )
                .after(tick::tick_step_motors),
        );
    schedule
}

fn run_core_game_loop(In(ticks): In<Vec<(u16, EntityActionMap)>>, world: &mut World) {
    for (_tick, tick_actions) in ticks {
        let mut inputs = world.resource_mut::<EntityActionMap>();
        inputs.0 = tick_actions.0;
        world.run_schedule(CoreTickSchedule);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, SystemSet)]
pub struct TickSet;

pub type TickActions = (u16, EntityActionMap);

pub struct TickPlugin<T, TM, S>
// the input tick_system type must:
//             take no In() parameter
//               |     return a vector containing all the inputs for each tick
//               |         |          (and it will have some parameters)
//               |         |            |
where
    //           v         v            v
    T: IntoSystem<(), Vec<TickActions>, TM> + Copy,
    //                                        ^^^^
    //                 and we need to be able to copy references to the function
    //                 (in practice, T should be a typical function)
    // additionally, we let the consumer pick a SystemSet
    S: FreeSystemSet + Copy,
{
    tick_system: T,
    tick_params_marker: PhantomData<TM>,
    tick_system_set: S,
}

impl<T, TM, S> TickPlugin<T, TM, S>
where
    T: IntoSystem<(), Vec<TickActions>, TM> + Copy,
    S: FreeSystemSet + Copy,
{
    #[must_use]
    pub fn new(tick_system_set: S, tick_system: T) -> Self {
        Self {
            tick_params_marker: PhantomData::<TM>,
            tick_system,
            tick_system_set,
        }
    }
}

// We also need to ensure that these generic parameters are Send + Sync + 'static
// because Bevy must be able to run systems from any thread
// Again, in practice, T should be defined as a "typical" function.
impl<T, TM, S> Plugin for TickPlugin<T, TM, S>
where
    T: IntoSystem<(), Vec<TickActions>, TM> + Copy + Send + Sync + 'static,
    TM: Send + Sync + 'static,
    S: FreeSystemSet + Copy,
{
    fn build(&self, app: &mut App) {
        app.init_resource::<EntityActionMap>()
            .add_schedule(CoreTickSchedule, build_core_tick_schedule())
            .add_system(
                self.tick_system
                    .pipe(run_core_game_loop)
                    .in_set(self.tick_system_set),
            );
    }
}
