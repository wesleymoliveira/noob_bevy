use bevy::prelude::*;

use crate::TILE_SIZE;

pub struct AsciiPlugin;

#[derive(Component)]
pub struct AsciiText;

#[derive(Copy, Clone)]
pub struct NineSliceIndices {
    center: usize,
    upper_left_index: usize,
    upper_right_index: usize,
    lower_left_index: usize,
    lower_right_index: usize,
    horizontal_index: usize,
    vertical_index: usize,
}

//for ease of use, I created my own resource wich will hold a copy of the handle. I t turns things ease for any system in the game to get its hands on this specific handle and to spawn a sprite from it
pub struct AsciiSpriteSheet(pub Handle<TextureAtlas>);

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_ascii_sprite_sheet)
            .insert_resource(NineSliceIndices {
                center: 2 * 16,
                upper_left_index: 13 * 16 + 10,
                upper_right_index: 11 * 16 + 15,
                lower_left_index: 12 * 16,
                lower_right_index: 13 * 16 + 9,
                horizontal_index: 12 * 16 + 4,
                vertical_index: 11 * 16 + 3,
            });
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
            Vec3::splat(1.0),
        ))
    }
    commands
        .spawn_bundle(SpatialBundle::default())
        .insert(Transform {
            translation: left_center,
            ..Default::default()
        })
        .insert(Name::new(format!("Text - {}", to_print)))
        .insert(AsciiText)
        .push_children(&character_sprites)
        .id()
}

pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    ascii: &AsciiSpriteSheet,
    index: usize,
    color: Color,
    translation: Vec3,
    scale: Vec3,
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
                scale: scale,
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

#[derive(Component)]
pub struct NineSlice;

pub fn spawn_nine_slice(
    commands: &mut Commands,
    ascii: &AsciiSpriteSheet,
    indices: &NineSliceIndices,
    width: f32,
    height: f32,
) -> Entity {
    assert!(width >= 2.0);
    assert!(height >= 2.0);

    let color = Color::rgb(0.3, 0.3, 0.9);
    let mut sprites = Vec::new();

    let left = (-width / 2.0 + 0.5) * TILE_SIZE;
    let right = (width / 2.0 - 0.5) * TILE_SIZE;
    let up = (height / 2.0 - 0.5) * TILE_SIZE;
    let down = (-height / 2.0 + 0.5) * TILE_SIZE;

    sprites.push(spawn_ascii_sprite(
        commands,
        ascii,
        indices.center,
        color,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(width - 2.0, height - 2.0, 0.0),
    ));
    sprites.push(spawn_ascii_sprite(
        commands,
        ascii,
        indices.upper_left_index,
        color,
        Vec3::new(left, up, 0.0),
        Vec3::splat(1.0),
    ));
    sprites.push(spawn_ascii_sprite(
        commands,
        ascii,
        indices.vertical_index,
        color,
        Vec3::new(left, 0.0, 0.0),
        Vec3::new(1.0, height - 2.0, 1.0),
    ));
    sprites.push(spawn_ascii_sprite(
        commands,
        ascii,
        indices.lower_left_index,
        color,
        Vec3::new(left, down, 0.0),
        Vec3::splat(1.0),
    ));
    sprites.push(spawn_ascii_sprite(
        commands,
        ascii,
        indices.horizontal_index,
        color,
        Vec3::new(0.0, down, 0.0),
        Vec3::new(width - 2.0, 1.0, 1.0),
    ));
    sprites.push(spawn_ascii_sprite(
        commands,
        ascii,
        indices.horizontal_index,
        color,
        Vec3::new(0.0, up, 0.0),
        Vec3::new(width - 2.0, 1.0, 1.0),
    ));
    sprites.push(spawn_ascii_sprite(
        commands,
        ascii,
        indices.upper_right_index,
        color,
        Vec3::new(right, up, 0.0),
        Vec3::splat(1.0),
    ));
    sprites.push(spawn_ascii_sprite(
        commands,
        ascii,
        indices.vertical_index,
        color,
        Vec3::new(right, 0.0, 0.0),
        Vec3::new(1.0, height - 2.0, 1.0),
    ));
    sprites.push(spawn_ascii_sprite(
        commands,
        ascii,
        indices.lower_right_index,
        color,
        Vec3::new(right, down, 0.0),
        Vec3::splat(1.0),
    ));

    commands
        .spawn_bundle(SpatialBundle::default())
        .insert(NineSlice)
        .insert(Name::new("NineSpriteBox"))
        .push_children(&sprites)
        .id()
}
