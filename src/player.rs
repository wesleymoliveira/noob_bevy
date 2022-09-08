use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;

//use TILE_SIZE to adjust the movement to be relative to it
use crate::{
    ascii::AsciiSpriteSheet,
    battle::BattleStats,
    fadeout::create_fadeout,
    graphics::{CharacterSheet, FacingDirection, FrameAnimation, PlayerGraphics},
    tilemap::{EncounterSpawner, TileCollider},
    GameState, MainCamera, TILE_SIZE,
};

pub struct PlayerPlugin;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct EncounterTracker {
    timer: Timer,
}

//make the player a unique component to be able to access it from all the entities  in the game, not a simple texture atlas sprite
#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
    pub active: bool,
    just_moved: bool,
    pub exp: usize,
}

impl Player {
    pub fn give_exp(&mut self, exp: usize, stats: &mut BattleStats) -> bool {
        self.exp += exp;
        if self.exp >= 50 {
            stats.health += 2;
            stats.max_health += 2;
            stats.attack += 1;
            stats.defense += 1;
            self.exp -= 50;
            return true;
        }
        false
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_resume(GameState::Overworld).with_system(show_player))
            .add_system_set(SystemSet::on_pause(GameState::Overworld).with_system(hide_player))
            .add_system_set(
                SystemSet::on_update(GameState::Overworld)
                    .with_system(player_encounter_checking.after("movement"))
                    //labelling to enforce right sort avoiding camera jittering when the player is moving
                    .with_system(camera_follow.after("movement"))
                    .with_system(player_movement.label("movement")),
            )
            .add_system_set(SystemSet::on_enter(GameState::Overworld).with_system(spawn_player));
    }
}

fn hide_player(
    mut player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    let mut player_vis = player_query.single_mut();
    player_vis.is_visible = false;

    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = false;
            }
        }
    }
}

fn show_player(
    mut player_query: Query<(&mut Player, &mut Visibility)>,
    children_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    let (mut player, mut player_vis) = player_query.single_mut();
    player.active = true;
    player_vis.is_visible = true;

    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = true;
            }
        }
    }
}

fn player_encounter_checking(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &mut EncounterTracker, &Transform)>,
    encounter_query: Query<&Transform, (With<EncounterSpawner>, Without<Player>)>,
    ascii: Res<AsciiSpriteSheet>,
    time: Res<Time>,
) {
    let (mut player, mut encounter_tracker, player_transform) = player_query.single_mut();
    let player_translation = player_transform.translation;

    if player.just_moved
        && encounter_query
            .iter()
            .any(|&transform| wall_collision_check(player_translation, transform.translation))
    {
        encounter_tracker.timer.tick(time.delta());

        if encounter_tracker.timer.finished() {
            player.active = false;
            create_fadeout(&mut commands, Some(GameState::Battle), &ascii);
        }
    }
}
fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<MainCamera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn player_movement(
    //used to move the player at a constant speed across different frame rates
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    //a query is a system fn param we use to look up groups of entities. On this situation we want to look up all the entities with the Player component
    mut player_query: Query<(&mut Player, &mut Transform, &mut PlayerGraphics)>,
    //query for tiles with colliders. The Player can have a collision with these tiles, then it could match two queries, it's the reason we are using Without<Player>
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
) {
    //as we have only one player it works fine, but if it returns more than one player, or zero we will have a problem
    let (mut player, mut transform, mut graphics) = player_query.single_mut();
    player.just_moved = false;

    if !player.active {
        return;
    }

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

    //fn any with a closure. -> it's like a for looping each tile in the query
    if !wall_query
        .iter()
        .any(|&transform| wall_collision_check(transform.translation, target))
    {
        if x_delta != 0.0 {
            player.just_moved = true;
            if x_delta > 0.0 {
                graphics.facing = FacingDirection::Right;
            } else {
                graphics.facing = FacingDirection::Left;
            }
        }
        transform.translation = target;
    }

    let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    if !wall_query
        .iter()
        .any(|&transform| wall_collision_check(transform.translation, target))
    {
        if y_delta != 0.0 {
            player.just_moved = true;
            if y_delta > 0.0 {
                graphics.facing = FacingDirection::Up;
            } else {
                graphics.facing = FacingDirection::Down;
            }
        }
        transform.translation = target;
    }
}

fn wall_collision_check(target_player_pos: Vec3, wall_translation: Vec3) -> bool {
    let collision = collide(
        target_player_pos,
        Vec2::splat(TILE_SIZE * 0.9),
        wall_translation,
        Vec2::splat(TILE_SIZE),
    );
    collision.is_some()
}

fn spawn_player(
    //commands as I expect to spawn an entitie
    mut commands: Commands,
    characters: Res<CharacterSheet>,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: characters.player_down[0],
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
            texture_atlas: characters.handle.clone(),
            ..default()
        })
        .insert(FrameAnimation {
            timer: Timer::from_seconds(0.2, true),
            frames: characters.player_down.to_vec(),
            current_frame: 0,
        })
        .insert(PlayerGraphics {
            facing: FacingDirection::Down,
        })
        .insert(Name::from("Player"))
        .insert(Player {
            speed: 3.0,
            active: true,
            just_moved: false,
            exp: 0,
        })
        .insert(BattleStats {
            health: 10,
            max_health: 10,
            attack: 2,
            defense: 1,
        })
        .insert(EncounterTracker {
            timer: Timer::from_seconds(2.0, true),
        });
}
