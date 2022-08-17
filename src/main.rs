#![allow(clippy::redundant_field_names)]
use bevy::prelude::*;
use bevy::render::{camera::ScalingMode, texture::ImageSettings};
use bevy::window::PresentMode;

mod player;
use player::PlayerPlugin;

mod debug;
use debug::DebugPlugin;

mod ascii;
use ascii::AsciiPlugin;

mod tilemap;
use tilemap::TileMapPlugin;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
pub enum GameState {
    Overworld,
    Battle,
}
fn main() {
    let height = 900.0;

    App::new()
        .add_state(GameState::Overworld)
        //bevy 0.8 now uses linear texture filtering by default, but we can change it's global default for textures that requires unfiltered pixels(pixel art).
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            title: "A Noob Bevy Game".to_string(),
            width: height * RESOLUTION,
            height: height,
            resizable: false,
            //will keep the frame rate usually around 60fps
            present_mode: PresentMode::AutoVsync,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_plugin(PlayerPlugin)
        .add_plugin(AsciiPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(TileMapPlugin)
        .run();
}

//commands run at the end of the frame, it is the place to put things that need to be done every frame, like a queue of tasks - Commands are executed after the game update logic runs, but before rendering occurs (in CoreStage::Update in the ECS schedule) . So if you spawn something with a command, it will be rendered without any delay. But if you want to access the spawned components, you will either need to access them after the CoreStage::Update stage (for the current frame), or wait until next frame.
fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.top = 1.;
    camera.projection.bottom = -1.;
    camera.projection.left = -1. * RESOLUTION;
    camera.projection.right = 1. * RESOLUTION;
    //to get a simple pixel art look
    camera.projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}
