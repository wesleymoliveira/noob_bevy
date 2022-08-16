use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;

//use TILE_SIZE to adjust the movement to be relative to it
use crate::{
    ascii::{spawn_ascii_sprite, AsciiSpriteSheet},
    tilemap::TileCollider,
    TILE_SIZE,
};

pub struct PlayerPlugin;

//make the player a unique component to be able to access it from all the entities  in the game, not a simple texture atlas sprite
#[derive(Component, Inspectable)]
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
    //query for tiles with colliders. The Player can have a collision with these tiles, then it could match two queries, it's the reason we are using Without<Player>
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
) {
    //as we have only one player it works fine, but if it returns more than one player, or zero we will have a problem
    let (player, mut transform) = player_query.single_mut();

    let mut y_delta = 0.0;

    if keyboard_input.pressed(KeyCode::W) {
        y_delta += time.delta_seconds() * TILE_SIZE * player.speed;
    }
    if keyboard_input.pressed(KeyCode::S) {
        y_delta -= time.delta_seconds() * TILE_SIZE * player.speed;
    }

    let mut x_delta = 0.0;
    if keyboard_input.pressed(KeyCode::A) {
        x_delta -= time.delta_seconds() * TILE_SIZE * player.speed;
    }
    if keyboard_input.pressed(KeyCode::D) {
        x_delta += time.delta_seconds() * TILE_SIZE * player.speed;
    }

    let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
    if wall_collision_check(target, &wall_query) {
        transform.translation = target;
    }

    let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    if wall_collision_check(target, &wall_query) {
        transform.translation = target;
    }
}

fn wall_collision_check(
    target_player_pos: Vec3,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>,
) -> bool {
    for wall_transform in wall_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(TILE_SIZE * 0.9),
            wall_transform.translation,
            Vec2::splat(TILE_SIZE),
        );
        if collision.is_some() {
            return false;
        }
    }
    true
}

fn spawn_player(
    //commands as I expect to spawn an entitie
    mut commands: Commands,
    ascii: Res<AsciiSpriteSheet>,
) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        1,
        Color::rgb(0.3, 0.3, 0.9),
        Vec3::new(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
    );

    commands
        .entity(player)
        .insert(Name::from("Player"))
        .insert(Player { speed: 3.0 });
    //.id(); //id() gives back the entity after creation

    let background = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        0,
        Color::rgb(0.5, 0.5, 0.5),
        Vec3::new(0.0, 0.0, -1.0),
    );

    commands.entity(background).insert(Name::new("Background"));
    //.id(); //id() gives back the entity after creation

    commands.entity(player).push_children(&[background]);
}
