use bevy::prelude::*;

use crate::TILE_SIZE;

pub struct AsciiPlugin;

//for ease of use, I created my own resource wich will hold a copy of the handle. I t turns things ease for any system in the game to get its hands on this specific handle and to spawn a sprite from it
pub struct AsciiSpriteSheet(pub Handle<TextureAtlas>);

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_ascii_sprite_sheet);
    }
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
