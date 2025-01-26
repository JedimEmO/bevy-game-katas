use avian2d::prelude::*;
use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use gamejam_platform_controller::{GameStates, PlayerPlugin, PlayerSpawnSettings};

pub struct SimplePlatformGame;

impl Plugin for SimplePlatformGame {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerSpawnSettings {
            position: Vec2::new(32., 16. * 16.),
        })
        .add_plugins((
            PhysicsPlugins::default().with_length_unit(16.),
            PlayerPlugin,
            LdtkPlugin,
        ))
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_int_cell::<WallBundle>(1)
        .add_systems(Startup, start_simple_platform_game)
        .add_systems(Update, (wall_spawn_system).run_if(in_state(GameStates::GameLoop)))
        .insert_resource(Gravity(Vec2::new(0., -9.81 * 32.)));

        #[cfg(feature = "avian-debug")]
        app.add_plugins(PhysicsDebugPlugin::default());
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

fn wall_spawn_system(
    mut commands: Commands,
    wall_query: Query<(Entity, &Wall, &GridCoords), Added<Wall>>,
) {
    for (entity, _wall, _coords) in wall_query.iter() {
        commands.entity(entity).insert((
            Collider::rectangle(16., 16.),
            CollisionLayers::new(0b00100, 0b00101),
            CollidingEntities::default(),
            RigidBody::Static,
            Friction::new(0.),
        ));
    }
}

fn start_simple_platform_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server
            .load("maps/grayboxes/nexus_ldtk/nexus.ldtk")
            .into(),
        ..default()
    });
}
