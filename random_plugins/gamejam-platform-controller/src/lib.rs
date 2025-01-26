use crate::input_systems::gamepad_input::gamepad_input_system;
use crate::input_systems::keyboard_input_system::keyboard_input_system;
use crate::player_systems::grounded_system::grounded_system;
use crate::player_systems::movement_dampening_system::movement_dampening_system;
use crate::player_systems::player_control_system::player_control_system;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use player_systems::player_spawn_system;

mod input_systems;
pub mod player_components;
mod player_const_rules;
pub mod player_systems;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameStates {
    #[default]
    Loading,
    SpawnPlayer,
    GameLoop,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameStates>()
            .add_loading_state(
                LoadingState::new(GameStates::Loading)
                    .continue_to_state(GameStates::SpawnPlayer)
                    .load_collection::<PlayerAssets>(),
            )
            .add_systems(
                OnEnter(GameStates::SpawnPlayer),
                player_spawn_system::spawn_player_system,
            )
            .add_event::<MovementAction>()
            .add_systems(Update, player_spawn_system::update_player_spawn)
            .add_systems(
                Update,
                (
                    grounded_system,
                    keyboard_input_system,
                    gamepad_input_system,
                    player_control_system,
                    movement_dampening_system,
                    animate_sprite_system,
                )
                    .run_if(in_state(GameStates::GameLoop))
                    .chain(),
            );

        setup_ldtk_entities(app);
    }
}

fn setup_ldtk_entities(app: &mut App) {
    app.register_ldtk_entity::<PlayerSpawnEntityBundle>("PlayerSpawn");
}

#[derive(Resource, Default)]
pub struct PlayerSpawnSettings {
    pub position: Vec2,
}

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 4, rows = 5))]
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

#[derive(Default, Component)]
struct PlayerSpawnEntity;

#[derive(Bundle, LdtkEntity, Default)]
struct PlayerSpawnEntityBundle {
    player_spawn: PlayerSpawnEntity,
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

#[derive(Event)]
pub enum MovementAction {
    Horizontal(Vec2),
    Jump,
    JumpAbort,
    Attack,
}
