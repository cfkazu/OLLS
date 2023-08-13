use std::collections::BinaryHeap;

use prelude::*;
mod components;
mod map_builder;
mod render_utils;
mod resources;
mod spawner;
mod states;
mod system;
mod tile;
mod ui;
mod user;

mod prelude {
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 80;
    pub use crate::components::*;
    pub use crate::map_builder::*;
    pub use crate::render_utils::*;
    pub use crate::resources::*;
    pub use crate::spawner::*;
    pub use crate::states::*;
    pub use crate::system::*;
    pub use crate::tile::*;
    pub use crate::ui::*;
    pub use crate::user::*;
    pub use bevy::prelude::*;
    pub use bevy::window::PrimaryWindow;
    pub use bevy::winit::WinitSettings;
    pub use bracket_lib::prelude::*;
}

const GRID_NUM: u32 = 826;
const WINDOW_SIZE: f32 = 500.0;
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("map1.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(48.0, 48.0), 33, 50, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(MapAsset {
        atlas: texture_atlas_handle.clone(),
    });

    let texture_handle: Handle<Image> = asset_server.load("chara_5.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(48.0, 48.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(CharacterAsset {
        atlas: texture_atlas_handle.clone(),
    });

    let texture_handle: Handle<Image> = asset_server.load("character.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(48.0, 48.0), 33, 25, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(MobAsset {
        atlas: texture_atlas_handle.clone(),
    });

    let texture_handle: Handle<Image> = asset_server.load("item.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(48.0, 48.0), 33, 25, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(ItemAsset {
        atlas: texture_atlas_handle.clone(),
    });

    let mut cam = Camera2dBundle::default();
    cam.transform.scale = Vec3::new(0.5, 0.5, 1.0);
    commands.spawn((MainCamera, cam));
    //let mymap = Map::testmap();
    let mymap = Map::load("town2");
    commands.insert_resource(mymap);

    let current_time = CurrentTime {
        time: components::Time {
            year: 2023,
            month: 08,
            day: 11,
            hour: 0,
            minute: 0,
            second: 0,
        },
    };
    commands.insert_resource(current_time);

    let queue: TurnQueue = TurnQueue {
        queue: BinaryHeap::new(),
    };
    commands.insert_resource(queue);
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Roguelike Game".to_string(),
                        resolution: (SCREEN_WIDTH as f32 * 10.0, SCREEN_HEIGHT as f32 * 10.0)
                            .into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_state::<TurnState>()
        .add_systems(Startup, setup)
        //.add_systems(PostStartup, test)
        .add_systems(PostStartup, spawn_map_tiles)
        .add_systems(PostStartup, spawn_player)
        //.add_systems(PostStartup,spawn_mobs)
        //.add_systems(PostStartup,spawn_map_templates)
        .add_systems(PostStartup, spawn_map_mob_items)
        .add_plugins(PlayerInputPlugin)
        .add_plugins(MobPlugin)
        .add_plugins(AwaitingInputPlugin)
        .add_plugins(TimePlugin)
        .add_plugins(SpawnerPlugin)
        .add_plugins(MapPlugin)
        //.add_systems(Update, movement::movement)
        .add_plugins(UIPlugin)
        .add_systems(PostUpdate, position_translation)
        .run();
}
