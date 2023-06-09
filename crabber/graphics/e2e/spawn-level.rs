use common_e2e::Test;

use bevy::prelude::{
    Added, BuildChildren, Color, Commands, Entity, IntoSystemAppConfig, OnEnter, Or,
    Query, SpatialBundle, Transform, Vec2,
};

use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, ShapeBundle, ShapePlugin, Stroke},
    shapes::{Circle, Rectangle, RectangleOrigin},
};

use crabber_protocol::{
    components::{Car, Controlled, Crab, Level, Raft},
    constants::TILE_SIZE_F32,
};

use crabber_graphics::{AssetsState, GraphicsPlugin as CrabGraphicsPlugin};

fn spawn_level(mut commands: Commands) {
    let level = Level::new_random();
    let (car_bundles, raft_bundles) = level.create_level_bundles();
    for bundle in car_bundles.into_iter() {
        commands.spawn((bundle, Controlled));
    }
    for bundle in raft_bundles.into_iter() {
        commands.spawn((bundle, Controlled));
    }
    commands.spawn(level);
}

fn main() {
    Test {
        label: "Test spawning level entities".to_string(),
        setup: |_app| {},
        setup_graphics: |app| {
            app.add_plugin(CrabGraphicsPlugin)
                .add_plugin(ShapePlugin)
                .add_system(spawn_level.in_schedule(OnEnter(AssetsState::Ready)))
                .add_system(handle_debug_graphic);
        },
        frames: 60,
        check: |_, _| true,
    }
    .run();
}

// a utility that attaches debug graphics to show where object transforms are centered
pub fn handle_debug_graphic(
    mut commands: Commands,
    new_game_object_query: Query<
        (Entity, &Transform, Option<&Crab>, Option<&Car>, Option<&Raft>),
        Or<(Added<Crab>, Added<Car>, Added<Raft>)>,
    >,
) {
    for (entity, transform, crab, car, raft) in new_game_object_query.iter() {
        let color = if crab.is_some() {
            Some(Color::BLUE)
        } else if car.is_some() {
            Some(Color::RED)
        } else if raft.is_some() {
            Some(Color::GREEN)
        } else {
            None
        };

        if let Some(color) = color {
            commands
                .entity(entity)
                .insert(SpatialBundle::from_transform(*transform))
                .with_children(|parent| {
                    let highlight = Rectangle {
                        origin: RectangleOrigin::Center,
                        extents: Vec2::new(TILE_SIZE_F32, TILE_SIZE_F32),
                    };
                    let anchor = Circle {
                        radius: 6.,
                        center: Vec2::ZERO,
                    };
                    let stroke_color = Color::rgba(color.r(), color.g(), color.b(), 0.7);
                    // a tile-sized box to define the boundaries of the tile
                    parent.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&highlight),
                            transform: Transform::from_xyz(0., 0., 1.),
                            ..Default::default()
                        },
                        Stroke::color(stroke_color),
                    ));
                    // a small filled dot to highlight the centerpoint
                    parent.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&anchor),
                            transform: Transform::from_xyz(0., 0., 1.),
                            ..Default::default()
                        },
                        Fill::color(color),
                    ));
                });
        }
    }
}
