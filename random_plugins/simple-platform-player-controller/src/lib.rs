use simple_2d_camera::PixelCameraTracked;
use avian2d::math::AdjustPrecision;
use avian2d::prelude::*;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::sprite::Anchor;

const COLLISION_MARGIN: f32 = 1.;
pub const TILE_SIZE_PIXELS: f32 = 16.;
const PLAYER_COLLIDER_SIZE: f32 = TILE_SIZE_PIXELS - COLLISION_MARGIN;

const MAX_SPEED: f32 = TILE_SIZE_PIXELS * 8.;
const ACCELERATION: f32 = 600.;
const JUMP_SPEED: f32 = 140.;
const AERIAL_X_DAMPENING_FACTOR: f32 = 0.3;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_system);
        app.add_event::<MovementAction>();
        app.add_systems(
            Update,
            (
                grounded_system,
                keyboard_input_system,
                player_move_action_system,
                movement_dampening_system,
            )
                .chain(),
        );
    }
}

#[derive(Component)]
#[require(
    Transform(|| Transform::from_xyz(0., 0., 10.)),
    RigidBody(|| RigidBody::Dynamic),
    Collider(|| Collider::rectangle(PLAYER_COLLIDER_SIZE, PLAYER_COLLIDER_SIZE)),
    CollisionMargin(|| CollisionMargin::from(COLLISION_MARGIN)),
    CollisionLayers(|| CollisionLayers::new(0b00001, 0b00101)),
    ExternalForce(|| ExternalForce::default().with_persistence(false)),
    ShapeCaster(|| ShapeCaster::new(Collider::rectangle(TILE_SIZE_PIXELS, TILE_SIZE_PIXELS), Vec2::ZERO, 0., Dir2::NEG_Y)),
    LockedAxes(|| LockedAxes::ROTATION_LOCKED),
    MovementDampeningFactor(|| MovementDampeningFactor(AERIAL_X_DAMPENING_FACTOR)),
    JumpState,
    PixelCameraTracked
)]
pub struct Player;

#[derive(Component)]
pub struct Grounded;

#[derive(Component, Default)]
pub struct JumpState {
    pub used: u8,
    pub left_ground_at_frame: Option<u32>
}

#[derive(Component)]
pub struct MovementDampeningFactor(pub f32);


#[derive(Resource, Default)]
pub struct PlayerSpawnSettings {
    pub position: Vec2
}

fn spawn_player_system(mut commands: Commands, asset_server: Res<AssetServer>, player_spawn_settings: Res<PlayerSpawnSettings>) {
    let sprite = asset_server.load("sprites/guy.png");

    commands.spawn((
        Player,
        Transform::from_xyz(player_spawn_settings.position.x, player_spawn_settings.position.y, 10.),
        Sprite {
            image: sprite,
            anchor: Anchor::Center,
            ..default()
        },
    ));
}

#[derive(Event)]
pub enum MovementAction {
    Horizontal(Vec2),
    Jump,
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

    if key_input.just_pressed(KeyCode::Space)  {
        event_sender.send(MovementAction::Jump);
    }
}

fn player_move_action_system(
    time: Res<Time>,
    frame_count: Res<FrameCount>,
    mut movement_events: EventReader<MovementAction>,
    mut player_velocity: Query<
        (&mut LinearVelocity, Option<&Grounded>, &mut JumpState, &ShapeHits),
        With<Player>,
    >,
) {
    let delta_t = time.delta_secs_f64().adjust_precision();

    for (mut linear_velocity, grounded, mut double_jumped, hits) in player_velocity.iter_mut() {
        for movement_action in movement_events.read() {
            match movement_action {
                MovementAction::Horizontal(dir) => {
                    let air_factor = if grounded.is_none() { 0.4 } else { 1. };
                    let reverse_factor = if linear_velocity.x.signum() != dir.x.signum() {
                        2.
                    } else {
                        1.
                    };

                    linear_velocity.x += dir.x * ACCELERATION * delta_t * air_factor * reverse_factor;
                    linear_velocity.y += dir.y * ACCELERATION * delta_t * air_factor * reverse_factor;

                    linear_velocity.x = linear_velocity.x.clamp(-MAX_SPEED, MAX_SPEED);
                }
                MovementAction::Jump => {
                    let left_ground_at = double_jumped.left_ground_at_frame.unwrap_or(frame_count.0.wrapping_sub(1000));
                    let is_grounded = grounded.is_some();

                    if is_grounded || frame_count.0.wrapping_sub(left_ground_at) < 30 {
                        linear_velocity.y = JUMP_SPEED;
                    }

                    double_jumped.used += 1;
                }
            }
        }
    }
}

fn grounded_system(
    mut commands: Commands,
    frame_count: Res<FrameCount>,
    mut query: Query<(Entity, &ShapeHits, &mut JumpState, &LinearVelocity), With<Player>>,
) {
    for (entity, hits, mut jump_state_data, velocity) in &mut query {
        let is_grounded = hits
            .iter()
            .any(|hit| hit.point2.y < 0. && hit.distance <= COLLISION_MARGIN);

        if is_grounded {
            commands.entity(entity).insert(Grounded);
            jump_state_data.used = 0;
            jump_state_data.left_ground_at_frame = None;
        } else {
            if jump_state_data.left_ground_at_frame.is_none() && velocity.y.abs() < 2. {
                jump_state_data.left_ground_at_frame = Some(frame_count.0);
            }

            commands.entity(entity).remove::<Grounded>();
        }
    }
}

fn movement_dampening_system(
    time: Res<Time>,
    mut query: Query<(
        &mut LinearVelocity,
        &MovementDampeningFactor,
        Option<&Grounded>
    )>,
) {
    for (mut velocity, dampening, grounded) in &mut query {
        if grounded.is_none() {
            velocity.x *= 1. - dampening.0 * time.delta_secs();
        }
    }
}
