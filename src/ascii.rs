use bevy::prelude::*;

use crate::TILE_SIZE;

pub struct AsciiPlugin;

#[derive(Component)]
pub struct AsciiText;

//for ease of use, I created my own resource wich will hold a copy of the handle. I t turns things ease for any system in the game to get its hands on this specific handle and to spawn a sprite from it
pub struct AsciiSpriteSheet(pub Handle<TextureAtlas>);

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_ascii_sprite_sheet);
    }
}

pub fn spawn_ascii_text(
    commands: &mut Commands,
    ascii: &AsciiSpriteSheet,
    to_print: &str,
    left_center: Vec3,
) -> Entity {
    let color = Color::rgb(0.8, 0.8, 0.8);
    let mut character_sprites = Vec::new();
    for (i, char) in to_print.chars().enumerate() {
        //https://doc.rust-lang.org/std/primitive.char.html#representation
        //char is always 4 bytes, spritesheet only has 256 images
        assert!(i < 256, "Index out of Ascii Range");

        character_sprites.push(spawn_ascii_sprite(
            commands,
            ascii,
            char as usize,
            color,
            Vec3 {
                x: i as f32 * TILE_SIZE,
                y: 0.0,
                z: 0.0,
            },
        ))
    }
    commands
        .spawn_bundle(SpatialBundle::default())
        .insert(Transform {
            translation: left_center,
            ..Default::default()
        })
        .insert(Name::new(format!("Text - {}", to_print)))
        .push_children(&character_sprites)
        .insert(AsciiText)
        .id()
}

pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    ascii: &AsciiSpriteSheet,
    index: usize,
    color: Color,
    translation: Vec3,
) -> Entity {
    assert!(index < 256, "Index out of Ascii Range");

    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = color;
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}

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
