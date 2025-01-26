use bevy::prelude::{Query, Res, Time};
use avian2d::prelude::LinearVelocity;
use crate::player_components::{Grounded, MovementDampeningFactor, Moving};

pub fn movement_dampening_system(
    time: Res<Time>,
    mut query: Query<(
        &mut LinearVelocity,
        &MovementDampeningFactor,
        Option<&Grounded>,
        Option<&Moving>,
    )>,
) {
    for (mut velocity, dampening, grounded, moving) in &mut query {
        if grounded.is_some() && moving.is_none() {
            velocity.x = 0.;
        } else {
            velocity.x *= 1. - dampening.0 * 0.15 * time.delta_secs();
        }
    }
}