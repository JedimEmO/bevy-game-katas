use avian2d::prelude::*;
use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use simple_platform_player_controller::player_components::PlayerSpawn;
use simple_platform_player_controller::{GameStates, Player, PlayerPlugin, PlayerSpawnSettings};

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
        .register_type::<PlayerSpawn>()
        .add_systems(Startup, start_simple_platform_game)
        .add_systems(Update, (wall_spawn_system, player_spawn_system).run_if(in_state(GameStates::GameLoop)))
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

fn player_spawn_system(
    mut spawn_settings: ResMut<PlayerSpawnSettings>,
    mut player: Query<&mut Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    spawn_entity: Query<&Transform, (Added<PlayerSpawn>, Without<Player>, Without<Camera>)>,
) {
    let Ok(spawn_transform) = spawn_entity.get_single() else {
        return;
    };

    info!("Spawning player at {:?}", spawn_transform.translation);
    spawn_settings.position.x = spawn_transform.translation.x;
    spawn_settings.position.y = spawn_transform.translation.y;

    if let Ok(mut player_transform) = player.get_single_mut() {
        player_transform.translation.x = spawn_transform.translation.x;
        player_transform.translation.y = spawn_transform.translation.y;
    };

    if let Ok(mut camera_transform) = camera.get_single_mut() {
        camera_transform.translation.x = spawn_transform.translation.x;
        camera_transform.translation.y = spawn_transform.translation.y;
    }
}
