use bevy::prelude::*;

//use TILE_SIZE to adjust the movement to be relative to it
use crate::{AsciiSpriteSheet, TILE_SIZE};

pub struct PlayerPlugin;

//make the player a unique component to be able to access it from all the entities  in the game, not a simple texture atlas sprite
#[derive(Component)]
pub struct Player {
    speed: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement);
    }
}

fn player_movement(
    //used to move the player at a constant speed across different frame rates
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    //a query is a system fn param we use to look up groups of entities. On this situation we want to look up all the entities with the Player component
    mut player_query: Query<(&Player, &mut Transform)>,
) {
    //as we have only one player it works fine, but if it returns more than one player, or zero we will have a problem
    let (player, mut transform) = player_query.single_mut();

    if keyboard_input.pressed(KeyCode::W) {
        transform.translation.y += time.delta_seconds() * TILE_SIZE * player.speed;
    }
    if keyboard_input.pressed(KeyCode::S) {
        transform.translation.y -= time.delta_seconds() * TILE_SIZE * player.speed;
    }
    if keyboard_input.pressed(KeyCode::A) {
        transform.translation.x -= time.delta_seconds() * TILE_SIZE * player.speed;
    }
    if keyboard_input.pressed(KeyCode::D) {
        transform.translation.x += time.delta_seconds() * TILE_SIZE * player.speed;
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
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

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
        .insert(Player { speed: 3.0 })
        .id(); //id() gives back the entity after creation

    let mut background_sprite = TextureAtlasSprite::new(0);
    background_sprite.color = Color::rgb(0.5, 0.5, 0.5);
    background_sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

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
