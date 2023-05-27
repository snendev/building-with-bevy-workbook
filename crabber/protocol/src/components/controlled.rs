use bevy_ecs::prelude::Component;

// When attached to an entity, this Component informs the core systems
// that the entity should be controlled by the core game loop behavior
#[derive(Component)]
pub struct Controlled;
