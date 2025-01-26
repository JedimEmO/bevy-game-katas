mod systems;

use bevy::prelude::*;
use simple_2d_camera::SimplePixel2dCameraPlugin;
use crate::systems::init_game::SimplePlatformGame;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SimplePixel2dCameraPlugin::default(),
        ))
        .add_plugins(SimplePlatformGame)
        .run();
}
