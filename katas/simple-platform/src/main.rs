mod game_system;
mod player;

use bevy::prelude::*;
use game_system::SimplePlatformGame;
use simple_2d_camera::SimplePixel2dCameraPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SimplePixel2dCameraPlugin::default(),
        ))
        .add_plugins(SimplePlatformGame)
        .run();
}
