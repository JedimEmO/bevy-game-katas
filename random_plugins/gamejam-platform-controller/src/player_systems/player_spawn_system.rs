use bevy::prelude::{Added, Commands, NextState, Query, Res, ResMut, Sprite, TextureAtlas, Timer, TimerMode, Transform};
use crate::{GameStates, PlayerAnimation, PlayerAssets, PlayerSpawnEntity, PlayerSpawnSettings};
use crate::player_components::Player;

pub fn spawn_player_system(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    player_spawn_settings: Res<PlayerSpawnSettings>,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    commands.spawn((
        Player,
        Transform::from_xyz(
            player_spawn_settings.position.x,
            player_spawn_settings.position.y,
            0.5,
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

    next_state.set(GameStates::GameLoop);
}

pub fn update_player_spawn(mut player_spawn_info: ResMut<PlayerSpawnSettings>, query: Query<&Transform, Added<PlayerSpawnEntity>>) {
    let Ok(transform) = query.get_single() else {
        return
    };
    
    player_spawn_info.position = transform.translation.truncate();
}