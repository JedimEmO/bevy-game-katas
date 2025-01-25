use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct PlayerSpawn;

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct GroundCollider;

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct WallCollider;
