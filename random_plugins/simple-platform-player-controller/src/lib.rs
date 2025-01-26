use avian2d::math::AdjustPrecision;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use simple_2d_camera::PixelCameraTracked;

pub mod player_components;

const COLLISION_MARGIN: f32 = 1.;
pub const TILE_SIZE_PIXELS: f32 = 16.;

const MAX_SPEED: f32 = TILE_SIZE_PIXELS * 20.;
const MAX_Y_SPEED: f32 = TILE_SIZE_PIXELS * 32.;
const MAX_JUMP_ACCELERATION_TIME: f64 = 0.25;
const ACCELERATION: f32 = 1200.;
const JUMP_SPEED: f32 = 350.;
const X_DAMPENING_FACTOR: f32 = 15.;
const FALL_GRAVITY: f32 = 10.0;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameStates {
    #[default]
    Loading,
    GameLoop,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameStates>()
            .add_loading_state(
                LoadingState::new(GameStates::Loading)
                    .continue_to_state(GameStates::GameLoop)
                    .load_collection::<PlayerAssets>(),
            )
            .add_systems(OnEnter(GameStates::GameLoop), spawn_player_system)
            .add_event::<MovementAction>()
            .add_systems(
                Update,
                (
                    grounded_system,
                    keyboard_input_system,
                    player_move_action_system,
                    movement_dampening_system,
                    animate_sprite_system,
                )
                    .run_if(in_state(GameStates::GameLoop))
                    .chain(),
            );
    }
}

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
    Friction(|| Friction::new(0.))
)]
pub struct Player;

#[derive(Component)]
pub struct Grounded;

#[derive(Component)]
pub struct Moving;

#[derive(Component, Default)]
pub struct JumpState {
    pub used: u8,
    pub left_ground_at: Option<f64>,
    pub last_grounded_time: Option<f64>,
}

#[derive(Component)]
pub struct MovementDampeningFactor(pub f32);

#[derive(Resource, Default)]
pub struct PlayerSpawnSettings {
    pub position: Vec2,
}

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 4, rows = 4))]
    player_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler(filter = nearest)))]
    #[asset(path = "sprites/guy.png")]
    player: Handle<Image>,
}

#[derive(Component)]
struct PlayerAnimation {
    timer: Timer,
    animation_row: usize,
    animation_count: usize,
}

fn animate_sprite_system(time: Res<Time>, mut query: Query<(&mut PlayerAnimation, &mut Sprite)>) {
    for (mut timer, mut sprite) in &mut query {
        timer.timer.tick(time.delta());

        if timer.timer.finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                timer.animation_count = (timer.animation_count + 1) % 4;
                atlas.index = timer.animation_row * 4 + timer.animation_count;
            }
        }
    }
}

fn spawn_player_system(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    player_spawn_settings: Res<PlayerSpawnSettings>,
) {
    commands.spawn((
        Player,
        Transform::from_xyz(
            player_spawn_settings.position.x,
            player_spawn_settings.position.y,
            0.,
        ),
        Sprite::from_atlas_image(
            player_assets.player.clone(),
            TextureAtlas::from(player_assets.player_layout.clone()),
        ),
        PlayerAnimation {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            animation_row: 0,
            animation_count: 0,
        },
    ));
}

#[derive(Event)]
pub enum MovementAction {
    Horizontal(Vec2),
    Jump,
    JumpAbort,
}

fn keyboard_input_system(
    mut event_sender: EventWriter<MovementAction>,
    key_input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec2::ZERO;

    if key_input.pressed(KeyCode::KeyD) || key_input.pressed(KeyCode::ArrowRight) {
        direction.x = 1.;
    } else if key_input.pressed(KeyCode::KeyA) || key_input.pressed(KeyCode::ArrowLeft) {
        direction.x = -1.;
    }

    if direction.length() > 0.1 {
        event_sender.send(MovementAction::Horizontal(direction));
    }

    if key_input.pressed(KeyCode::Space) {
        event_sender.send(MovementAction::Jump);
    }

    if key_input.just_released(KeyCode::Space) {
        event_sender.send(MovementAction::JumpAbort);
    }
}

fn player_move_action_system(
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
        ),
        With<Player>,
    >,
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
            }
        }
    }
}

fn grounded_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &ShapeHits,
            &mut JumpState,
            &LinearVelocity,
            &mut PlayerAnimation,
            &Transform,
        ),
        With<Player>,
    >,
    spatial_query: SpatialQuery,
) {
    for (entity, hits, mut jump_state_data, velocity, mut animation, player_transform) in &mut query
    {
        let is_grounded = hits.iter().any(|hit| {
            hit.point2.y < 0.
                && hit.distance <= 18.
                && hit.normal1.y >= 0.95
                && hit.normal2.y <= -0.95
        });

        let now = time.elapsed_secs_f64();

        if is_grounded {
            jump_state_data.last_grounded_time = Some(now);

            if velocity.y >= 0. {
                commands.entity(entity).insert(Grounded);
                jump_state_data.used = 0;
                jump_state_data.left_ground_at = None;
                animation.animation_row = 0;
            }
        } else {
            // Check for collisions when going up
            if velocity.y < 0. {
                let up_hits = spatial_query.ray_hits(
                    player_transform.translation.truncate(),
                    Dir2::Y,
                    50.,
                    2,
                    true,
                    &SpatialQueryFilter::default(),
                );

                if up_hits.iter().any(|hit| hit.distance < 18.) {
                    jump_state_data.left_ground_at = Some(0.);
                }
            }

            commands.entity(entity).remove::<Grounded>();
            animation.animation_row = 3;
        }
    }
}

fn movement_dampening_system(
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
