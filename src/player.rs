use bevy::prelude::*;

use crate::AsciiSpriteSheet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
    }
}

fn spawn_player(
    //commands as I expect to spawn an entitie
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
