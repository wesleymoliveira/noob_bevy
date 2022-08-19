use bevy::prelude::*;

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSpriteSheet},
    fadeout::create_fadeout,
    GameState,
};

#[derive(Component)]
pub struct Enemy;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Battle)
                .with_system(test_exit_battle)
                .with_system(battle_camera),
        )
        .add_system_set(SystemSet::on_enter(GameState::Battle).with_system(spawn_enemy))
        .add_system_set(SystemSet::on_exit(GameState::Battle).with_system(despawn_enemy));
    }
}

fn battle_camera(mut camera_query: Query<&mut Transform, With<Camera>>) {
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = 0.0;
    camera_transform.translation.y = 0.0;
}

fn spawn_enemy(mut commands: Commands, ascii: Res<AsciiSpriteSheet>) {
    let sprite = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        'b' as usize,
        Color::rgb(0.8, 0.8, 0.8),
        Vec3::new(0.0, 0.5, 100.0),
    );
    commands
        .entity(sprite)
        .insert(Enemy)
        .insert(Name::new("Bat"));
}

fn despawn_enemy(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for entity in enemy_query.iter() {
        //despawn_recursive removes the entity and all of its children
        commands.entity(entity).despawn_recursive();
    }
}

fn test_exit_battle(
    mut commands: Commands,
    keyboard: ResMut<Input<KeyCode>>,
    ascii: Res<AsciiSpriteSheet>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        create_fadeout(&mut commands, GameState::Overworld, &ascii);
    }
}
