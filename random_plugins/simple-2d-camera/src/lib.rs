use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub const CAMERA_TRACK_SPEED: f32 = 200.;
pub struct SimplePixel2dCameraPlugin {
    pub screen_size: Vec2,
}

impl Default for SimplePixel2dCameraPlugin {
    fn default() -> Self {
        Self {
            screen_size: Vec2::new(480.0, 270.0),
        }
    }
}

#[derive(Resource)]
struct PixelCameraResolution(Vec2);

#[derive(Component, Default)]
pub struct PixelCameraTracked;

impl Plugin for SimplePixel2dCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PixelCameraResolution(self.screen_size));
        app.add_systems(Startup, start_camera_system);
        app.add_systems(Update, camera_track_system);
    }
}

fn camera_track_system(
    time: Res<Time>,
    mut camera: Query<&mut Transform, With<Camera>>,
    tracked: Query<&Transform, (With<PixelCameraTracked>, Without<Camera>)>,
) {
    let mut camera = camera.single_mut();

    for transform in tracked.iter() {
        let dx = transform.translation.x - camera.translation.x;
        let dy = transform.translation.y - camera.translation.y;

        if dx.abs() >= 50. {
            camera.translation.x += dx.signum() * CAMERA_TRACK_SPEED * time.delta_secs();
        }

        if dy.abs() >= 30. {
            camera.translation.y += dy.signum() * CAMERA_TRACK_SPEED * time.delta_secs();
        }
    }
}

fn start_camera_system(mut commands: Commands, camera_resolution: Res<PixelCameraResolution>) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: camera_resolution.0.x,
                height: camera_resolution.0.y,
            },
            near: -1000.,
            far: 1000.,
            ..OrthographicProjection::default_2d()
        },
    ));
}
