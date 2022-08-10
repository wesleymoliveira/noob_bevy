use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::PresentMode;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    let height = 900.0;

    App::new()
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
        .add_startup_system(spawn_camera)
        .add_plugins(DefaultPlugins)
        .run();
}

//commands run at the end of the frame, it is the place to put things that need to be done every frame, like a queue of tasks
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
