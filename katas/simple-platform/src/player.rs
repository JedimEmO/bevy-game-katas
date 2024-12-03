use bevy::prelude::*;
use bevy::sprite::Anchor;
use avian2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_system);
        app.add_systems(Update, player_input_system);
    }
}

#[derive(Component)]
#[require(
    Transform(|| Transform::from_xyz(0., 0., 10.)),
    RigidBody(|| RigidBody::Dynamic),
    Collider(|| Collider::circle(10.)),
    Mass(|| Mass::new(1.))
)]
pub struct Player;

fn spawn_player_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite = asset_server.load("sprites/guy.png");

    commands.spawn((
        Player,
        Transform::from_xyz(0., 0., 0.),
        Sprite {
            image: sprite,
            anchor: Anchor::Center,
            ..default()
        },
    ));
}

fn player_input_system(
    key_input: Res<ButtonInput<KeyCode>>,
    mut player_velocity: Query<&mut LinearVelocity, With<Player>>,
) {
    for mut velocity in player_velocity.iter_mut() {
        if key_input.just_pressed(KeyCode::Space) {
            velocity.0.y = 50.;
        }
    }
}
