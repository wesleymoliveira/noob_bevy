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
        .add_startup_system(spawn_player)
        // I want this to load before any other startup systems
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii_sprite_sheet)
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

//for ease of use, I created my own resource wich will hold a copy of the handle. I t turns things ease for any system in the game to get its hands on this specific handle and to spawn a sprite from it
struct AsciiSpriteSheet(Handle<TextureAtlas>);

fn load_ascii_sprite_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    //it's recommended to use a texture atlas to reduce the number of textures loaded
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    //all we need is a map of all the loaded assets of the texture atlas type

    let image = asset_server.load("Ascii.png");

    //I'm using padding around each tile to prevent bleeding of pixels from adjacent tiles
    let texture_atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(9.0),
        16,
        16,
        Vec2::splat(2.0),
        Vec2::ZERO,
    );

    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(AsciiSpriteSheet(atlas_handle));
}

fn spawn_player(
    //again commands as I expect to spawn an entitie
    mut commands: Commands,
    ascii: Res<AsciiSpriteSheet>,
) {
    //sprite of a smile face
    let mut sprite = TextureAtlasSprite::new(1);

    sprite.color = Color::rgb(0.3, 0.3, 0.9);
    sprite.custom_size = Some(Vec2::splat(1.0));

    //spawning a spritesheet bundle in the center and gives it a copy of atlas handle
    let player = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 900.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::from("Player"))
        .id(); //id() gives back the entity after creation

    let mut background_sprite = TextureAtlasSprite::new(0);
    background_sprite.color = Color::rgb(0.5, 0.5, 0.5);
    background_sprite.custom_size = Some(Vec2::splat(1.0));

    let background = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: background_sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Background"))
        .id(); //id() gives back the entity after creation

    commands.entity(player).push_children(&[background]);
}
