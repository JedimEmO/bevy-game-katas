use avian2d::prelude::*;
use avian2d::PhysicsPlugins;
use bevy::ecs::observer::TriggerTargets;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use simple_platform_player_controller::{Player, PlayerPlugin};

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
        #[cfg(feature = "avian-debug")]
        app.add_plugins(PhysicsDebugPlugin::default());
        app.add_systems(Startup, start_simple_platform_game);
        app.add_systems(
            Update,
            (collectible_system, player_collectible_collider_system),
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
                commands.entity(collider.entity).insert(Friction::new(1.));
                collider
            })
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
struct Collectible {}

fn collectible_system(
    mut commands: Commands,
    sprites: Res<GameSpriteSheet>,
    collectibles: Query<(Entity, &Transform), Added<Collectible>>,
) {
    for (entity, transform) in collectibles.iter() {
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
