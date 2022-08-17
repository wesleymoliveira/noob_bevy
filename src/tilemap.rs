use bevy::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSpriteSheet},
    TILE_SIZE,
};

#[derive(Component)]
pub struct EncounterSpawner;

pub struct TileMapPlugin;

#[derive(Component)]
pub struct TileCollider;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_simple_map);
    }
}

fn create_simple_map(mut commands: Commands, ascii: Res<AsciiSpriteSheet>) {
    let file = File::open("assets/map.txt").expect("No map file found");
    let mut tiles = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let tile = spawn_ascii_sprite(
                    &mut commands,
                    &ascii,
                    char as usize,
                    Color::WHITE,
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
                );
                if char == '#' {
                    commands.entity(tile).insert(TileCollider);
                }

                if char == '~' {
                    commands.entity(tile).insert(EncounterSpawner);
                }
                tiles.push(tile);
            }
        }
    }

    commands
        //bevy now on 0.8 added the visibility inheritance. check it out here https://bevyengine.org/news/bevy-0-8/#spatialbundle-and-visibilitybundle
        .spawn_bundle(SpatialBundle::default())
        .insert(Name::from("Map"))
        /*
        SpatialBundle propagates Transform GlobalTransform, Visibility and ComputedVisibility down to the children
        .insert(Transform::default())
        .insert(GlobalTransform::default()) */
        .push_children(&tiles);
}
