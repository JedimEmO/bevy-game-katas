use avian2d::prelude::*;
use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use simple_platform_player_controller::PlayerPlugin;

pub struct SimplePlatformGame;

impl Plugin for SimplePlatformGame {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::default().with_length_unit(16.),
            PlayerPlugin
        ));
        #[cfg(feature = "avian-debug")]
        app.add_plugins(PhysicsDebugPlugin::default());
        app.add_systems(Startup, start_simple_platform_game);
        app.insert_resource(Gravity(Vec2::new(0., -9.81 * 32.)));
    }
}

fn start_simple_platform_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let tile_mesh = Mesh::from(Rectangle::new(16., 16.));
    let tile_mesh = meshes.add(tile_mesh);
    let material = materials.add(ColorMaterial::from_color(
        bevy::color::palettes::tailwind::EMERALD_800,
    ));

    commands.spawn((
        Tile,
        Transform::from_xyz(0., -40., 0.),
        Mesh2d(tile_mesh.clone()),
        MeshMaterial2d(material.clone()),
    ));

    commands.spawn((
        Tile,
        Transform::from_xyz(0., 40., 0.),
        Mesh2d(tile_mesh.clone()),
        MeshMaterial2d(material.clone()),
    ));
    commands.spawn((
        Tile,
        Transform::from_xyz(16., -40., 0.),
        Mesh2d(tile_mesh.clone()),
        MeshMaterial2d(material.clone()),
    ));
    commands.spawn((
        Tile,
        Transform::from_xyz(-16., -40., 0.),
        Mesh2d(tile_mesh.clone()),
        MeshMaterial2d(material.clone()),
    ));

    commands.spawn((
        Tile,
        Transform::from_xyz(-32., -40. + 16., 0.),
        Mesh2d(tile_mesh.clone()),
        MeshMaterial2d(material.clone()),
    ));

    commands.spawn((
        Tile,
        Transform::from_xyz(32., -40. + 16., 0.),
        Mesh2d(tile_mesh.clone()),
        MeshMaterial2d(material.clone()),
    ));
}

#[derive(Component)]
#[require(
    Transform,
    Collider(|| Collider::rectangle(16., 16.)),
    RigidBody(|| RigidBody::Static),
    Friction(|| Friction::new(0.2))
)]
struct Tile;

