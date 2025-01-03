mod game_system;

use bevy::prelude::*;
use game_system::SimplePlatformGame;
use simple_2d_camera::SimplePixel2dCameraPlugin;
use simple_platform_player_controller::{PlayerSpawnSettings};

fn main() {
    App::new()
        .insert_resource(PlayerSpawnSettings::default())
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SimplePixel2dCameraPlugin::default(),
        ))
        .add_plugins(SimplePlatformGame)
        .run();
}
