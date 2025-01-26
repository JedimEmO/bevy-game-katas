use avian2d::prelude::*;
use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use simple_platform_player_controller::player_components::PlayerSpawn;
use simple_platform_player_controller::{Player, PlayerPlugin, PlayerSpawnSettings};

pub struct SimplePlatformGame;

impl Plugin for SimplePlatformGame {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::default().with_length_unit(16.),
            PlayerPlugin,
            TilemapPlugin,
            TiledMapPlugin::default(),
            TiledPhysicsPlugin::<StaticTiledAvianBackend>::default(),
        ));

        app.register_type::<Collectible>();
        app.register_type::<PlayerSpawn>();
        #[cfg(feature = "avian-debug")]
        app.add_plugins(PhysicsDebugPlugin::default());
        app.add_systems(Startup, start_simple_platform_game);
        app.add_systems(
            Update,
            (
                collectible_system,
                player_collectible_collider_system,
                player_spawn_system,
            ),
        );
        app.insert_resource(Gravity(Vec2::new(0., -9.81 * 32.)));
    }
}

#[derive(Resource)]
pub struct GameSpriteSheet {
    pub coin: Handle<Image>,
}

fn start_simple_platform_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    let coin_handle: Handle<Image> = asset_server.load("katas/2/sprites/coin.png");

    commands.insert_resource(GameSpriteSheet { coin: coin_handle });

    let map_handle: Handle<TiledMap> = asset_server.load("katas/2/map.rendered.tmx");
    commands.spawn(TiledMapHandle(map_handle));
}

#[derive(Default)]
struct StaticTiledAvianBackend(TiledPhysicsAvianBackend);

impl TiledPhysicsBackend for StaticTiledAvianBackend {
    fn spawn_collider(
        &self,
        commands: &mut Commands,
        map: &tiled::Map,
        collider_source: &TiledColliderSource,
    ) -> Option<TiledColliderSpawnInfos> {
        self.0
            .spawn_collider(commands, map, collider_source)
            .map(|collider| {
                commands.entity(collider.entity).insert(RigidBody::Static);
                commands.entity(collider.entity).insert(Friction::new(0.));
                collider
            })
    }
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

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
struct Collectible {}

fn collectible_system(
    mut commands: Commands,
    sprites: Res<GameSpriteSheet>,
    collectibles: Query<Entity, Added<Collectible>>,
) {
    for entity in collectibles.iter() {
        commands.entity(entity).insert((
            Sprite {
                image: sprites.coin.clone(),
                ..default()
            },
            Collider::circle(4.),
            CollisionLayers::new(0b00100, 0b00101),
            CollidingEntities::default(),
        ));
    }
}

fn player_collectible_collider_system(
    mut commands: Commands,
    colliders: Query<(Entity, &CollidingEntities)>,
    player: Query<Entity, With<Player>>,
) {
    let Some(player_entity) = player.iter().next() else {
        return;
    };

    for (entity, collisions) in colliders.iter() {
        for c in collisions.iter() {
            if *c == player_entity {
                info!("Player collected a collectible!");
                commands.entity(entity).despawn();
            }
        }
    }
}
