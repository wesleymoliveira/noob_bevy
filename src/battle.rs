use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{
    ascii::{spawn_ascii_sprite, spawn_ascii_text, AsciiSpriteSheet, AsciiText},
    fadeout::create_fadeout,
    player::Player,
    GameState, TILE_SIZE,
};

#[derive(Component)]
pub struct Enemy;

pub struct BattlePlugin;

pub struct FightEvent {
    target: Entity,
    damage_amount: isize,
}

#[derive(Component, Inspectable)]
pub struct BattleStats {
    pub health: isize,
    pub max_health: isize,
    pub attack: isize,
    pub defense: isize,
}

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FightEvent>()
            .add_system_set(
                SystemSet::on_update(GameState::Battle)
                    .with_system(test_exit_battle)
                    .with_system(battle_input)
                    .with_system(damage_calculation)
                    .with_system(battle_camera),
            )
            .add_system_set(SystemSet::on_enter(GameState::Battle).with_system(spawn_enemy))
            .add_system_set(SystemSet::on_exit(GameState::Battle).with_system(despawn_enemy));
    }
}

fn damage_calculation(
    mut commands: Commands,
    ascii: Res<AsciiSpriteSheet>,
    mut fight_event: EventReader<FightEvent>,
    text_query: Query<&AsciiText>,
    mut target_query: Query<(&Children, &mut BattleStats)>,
) {
    for event in fight_event.iter() {
        let (target_children, mut target_stats) = target_query
            .get_mut(event.target)
            .expect("Fight target with stats not found");

        target_stats.health = std::cmp::max(
            target_stats.health - event.damage_amount - target_stats.defense,
            0,
        );

        //Update health
        for child in target_children.iter() {
            //See if this child is the health text
            if text_query.get(*child).is_ok() {
                //Delete old text
                commands.entity(*child).despawn_recursive();
                //Create new text
                let new_health = spawn_ascii_text(
                    &mut commands,
                    &ascii,
                    &format!("Health: {}", target_stats.health as usize),
                    //relative to enemy pos
                    Vec3::new(-4.5 * TILE_SIZE, 2.0 * TILE_SIZE, 100.0),
                );

                commands.entity(event.target).add_child(new_health);
            }
        }

        if target_stats.health == 0 {
            create_fadeout(&mut commands, GameState::Overworld, &ascii);
            /*  commands.despawn(event.target); */
        }
    }
}

fn battle_input(
    keyboard: ResMut<Input<KeyCode>>,
    mut fight_event: EventWriter<FightEvent>,
    player_query: Query<&BattleStats, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    let player_stats = player_query.single();
    //todo handle multiple enemies and enemy selection

    let target = enemy_query.iter().next().unwrap();
    if keyboard.just_pressed(KeyCode::Return) {
        fight_event.send(FightEvent {
            target,
            damage_amount: player_stats.attack,
        });
    }
}

fn battle_camera(mut camera_query: Query<&mut Transform, With<Camera>>) {
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = 0.0;
    camera_transform.translation.y = 0.0;
}

fn spawn_enemy(mut commands: Commands, ascii: Res<AsciiSpriteSheet>) {
    let enemy_health = 3;
    let health_text = spawn_ascii_text(
        &mut commands,
        &ascii,
        &format!("Health: {}", enemy_health as usize),
        //relative to enemy pos
        Vec3::new(-4.5 * TILE_SIZE, 2.0 * TILE_SIZE, 100.0),
    );
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
        .insert(BattleStats {
            health: enemy_health,
            max_health: enemy_health,
            attack: 2,
            defense: 1,
        })
        .insert(Name::new("Bat"))
        .add_child(health_text);
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
