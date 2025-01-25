mod game_system;

use bevy::prelude::*;
use game_system::SimplePlatformGame;
use simple_2d_camera::SimplePixel2dCameraPlugin;
use simple_platform_player_controller::{PlayerSpawnSettings, TILE_SIZE_PIXELS};

fn main() {
    App::new()
        .insert_resource(PlayerSpawnSettings { position: Vec2::new(2. * TILE_SIZE_PIXELS, 10. * TILE_SIZE_PIXELS) })
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SimplePixel2dCameraPlugin::default(),
        ))
        .add_plugins(SimplePlatformGame)
        .run();
}
