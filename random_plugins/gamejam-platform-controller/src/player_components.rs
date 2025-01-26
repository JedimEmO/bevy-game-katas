use avian2d::prelude::*;
use bevy::prelude::*;
use simple_2d_camera::PixelCameraTracked;

use crate::player_const_rules::*;

#[derive(Component)]
#[require(
    Transform(|| Transform::from_xyz(32., 0., 0.)),
    RigidBody(|| RigidBody::Dynamic),
    Collider(|| Collider::rectangle(7., 30.)),
    CollisionMargin(|| CollisionMargin::from(COLLISION_MARGIN)),
    CollisionLayers(|| CollisionLayers::new(0b00001, 0b00101)),
    ExternalForce(|| ExternalForce::default().with_persistence(false)),
    GravityScale,
    ShapeCaster(|| ShapeCaster::new(Collider::rectangle(4., 4.), Vec2::ZERO, 0., Dir2::NEG_Y)),
    LockedAxes(|| LockedAxes::ROTATION_LOCKED),
    MovementDampeningFactor(|| MovementDampeningFactor(X_DAMPENING_FACTOR)),
    JumpState,
    PixelCameraTracked,
    Friction(|| Friction::new(0.)),
    PlayerActionTracker
)]
pub struct Player;

#[derive(Component)]
pub struct Grounded;

#[derive(Component)]
pub struct Moving;

#[derive(Component)]
pub struct Attacking {
    pub attack_started_at: f64,
}

#[derive(Component, Default)]
pub struct PlayerActionTracker {
    pub last_attack_at: Option<f64>,
}

#[derive(Component, Default)]
pub struct JumpState {
    pub used: u8,
    pub left_ground_at: Option<f64>,
    pub last_grounded_time: Option<f64>,
}

#[derive(Component)]
pub struct MovementDampeningFactor(pub f32);