use crate::player_components::{
    Attacking, Grounded, JumpState, Moving, Player, PlayerActionTracker,
};
use crate::player_const_rules::{ACCELERATION, FALL_GRAVITY, JUMP_SPEED, MAX_JUMP_ACCELERATION_TIME, MAX_SPEED, MAX_Y_SPEED, PLAYER_ATTACK_DELAY_SECONDS, WALL_HIT_KICKBACK_ACCELERATION};
use crate::{MovementAction, PlayerAnimation};
use avian2d::math::AdjustPrecision;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_trauma_shake::Shake;
use simple_2d_camera::CameraShake;

pub fn player_control_system(
    mut commands: Commands,
    time: Res<Time>,
    mut movement_events: EventReader<MovementAction>,
    mut player_velocity: Query<
        (
            Entity,
            &mut LinearVelocity,
            Option<&Grounded>,
            &mut JumpState,
            &mut GravityScale,
            &mut PlayerAnimation,
            &mut Sprite,
            Option<&Attacking>,
            &mut PlayerActionTracker,
            &Transform,
        ),
        With<Player>,
    >,
    mut camera_query: Query<&mut Shake, With<Camera>>,
    spatial_query: SpatialQuery,
) {
    let delta_t = time.delta_secs_f64().adjust_precision();

    for (
        entity,
        mut linear_velocity,
        grounded,
        mut jump_state,
        mut gravity_scale,
        mut animation,
        mut sprite,
        attacking,
        mut player_actions,
        player_transform,
    ) in player_velocity.iter_mut()
    {
        if !grounded.is_some() {
            gravity_scale.0 = FALL_GRAVITY;
        } else {
            gravity_scale.0 = 1.0;
        }

        if linear_velocity.y.abs() >= MAX_Y_SPEED {
            linear_velocity.y = linear_velocity.y.clamp(-MAX_Y_SPEED, MAX_Y_SPEED);
        }

        if let Some(_attacking) = attacking {
            if animation.animation_count == 3 {
                animation.timer = Timer::from_seconds(0.1, TimerMode::Repeating);
                animation.animation_row = 0;
                commands.entity(entity).remove::<Attacking>();
            }

            continue;
        }

        if movement_events.is_empty() {
            commands.entity(entity).remove::<Moving>();
            continue;
        } else {
            commands.entity(entity).insert(Moving);
        }

        for movement_action in movement_events.read() {
            match movement_action {
                MovementAction::Horizontal(dir) => {
                    if grounded.is_some() {
                        animation.animation_row = 1;
                    }

                    let reverse_factor = if linear_velocity.x.signum() != dir.x.signum() {
                        FALL_GRAVITY
                    } else {
                        1.
                    };

                    linear_velocity.x += dir.x * ACCELERATION * delta_t * reverse_factor;
                    linear_velocity.y += dir.y * ACCELERATION * delta_t * reverse_factor;

                    linear_velocity.x = linear_velocity.x.clamp(-MAX_SPEED, MAX_SPEED);
                    sprite.flip_x = dir.x < 0.;
                }
                MovementAction::Jump => {
                    let now = time.elapsed_secs_f64();
                    let left_ground_at = jump_state.left_ground_at;
                    let is_grounded = grounded.is_some();

                    let coyote_time_delta = now - jump_state.last_grounded_time.unwrap_or(0.);
                    let can_coyote_jump = coyote_time_delta <= 0.2;

                    if is_grounded || can_coyote_jump && jump_state.used == 0 {
                        jump_state.used = 1;
                        jump_state.left_ground_at = Some(now);
                        linear_velocity.y = JUMP_SPEED;
                        gravity_scale.0 = 1.;
                    } else if left_ground_at.is_some()
                        && now - left_ground_at.unwrap() < MAX_JUMP_ACCELERATION_TIME
                    {
                        linear_velocity.y = JUMP_SPEED;
                        gravity_scale.0 = 1.;
                    }

                    jump_state.used += 1;
                }
                MovementAction::JumpAbort => {
                    let now = time.elapsed_secs_f64();
                    if let Some(left_ground_at) = jump_state.left_ground_at {
                        if now - left_ground_at < 0.3 {
                            gravity_scale.0 = FALL_GRAVITY;
                        }
                    }
                }
                MovementAction::Attack => {
                    let now = time.elapsed_secs_f64();

                    if now - player_actions.last_attack_at.unwrap_or(0.)
                        < PLAYER_ATTACK_DELAY_SECONDS
                    {
                        continue;
                    }

                    player_actions.last_attack_at = Some(now);

                    animation.animation_row = 4;
                    animation.animation_count = 0;
                    animation.timer = Timer::from_seconds(0.020, TimerMode::Repeating);

                    commands.entity(entity).insert(Attacking {
                        attack_started_at: now,
                    });

                    let facing_direction = if sprite.flip_x { -1. } else { 1. };

                    // Process hits
                    let hits = spatial_query.ray_hits(
                        player_transform.translation.truncate(),
                        Dir2::from_xy(1. * facing_direction, 0.).unwrap(),
                        10.,
                        2,
                        true,
                        &SpatialQueryFilter::default()
                    );

                    if hits.len() > 1 {
                        let kickback = Vec2::new(facing_direction * -0.3, 0.5).normalize();

                        linear_velocity.x += kickback.x * WALL_HIT_KICKBACK_ACCELERATION;
                        linear_velocity.y += kickback.y * WALL_HIT_KICKBACK_ACCELERATION;


                        if let Ok(mut camera_entity) = camera_query.get_single_mut() {
                            camera_entity.add_trauma(0.2);
                        }
                    }
                }
            }
        }
    }
}
