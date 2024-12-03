use avian2d::prelude::*;
use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use crate::player::{PlayerPlugin};

pub struct SimplePlatformGame;

impl Plugin for SimplePlatformGame {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::default().with_length_unit(10.),
            PlayerPlugin
        ));
        app.add_systems(Startup, start_simple_platform_game);
        app.insert_resource(Gravity(Vec2::new(0., -98.1)));
    }
}

fn start_simple_platform_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let tile_mesh = Mesh::from(Rectangle::new(15., 15.));
    let tile_mesh = meshes.add(tile_mesh);
    let material = materials.add(ColorMaterial::from_color(
        bevy::color::palettes::tailwind::EMERALD_800,
    ));

    commands.spawn((
        Tile,
        Transform::from_xyz(0., -40., 0.),
        Mesh2d(tile_mesh),
        MeshMaterial2d(material),
    ));
}

#[derive(Component)]
#[require(Transform, Collider(|| Collider::rectangle(15., 15.)), RigidBody(|| RigidBody::Static))]
struct Tile;

