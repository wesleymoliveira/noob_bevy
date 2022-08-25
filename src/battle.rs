use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{
    ascii::{
        spawn_ascii_sprite, spawn_ascii_text, spawn_nine_slice, AsciiSpriteSheet, AsciiText,
        NineSlice, NineSliceIndices,
    },
    fadeout::create_fadeout,
    player::Player,
    GameState, RESOLUTION, TILE_SIZE,
};

#[derive(Component)]
pub struct Enemy;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum BattleState {
    PlayerTurn,
    PlayerAttack,
    EnemyTurn(bool),
    EnemyAttack,
    Exiting,
    Reward,
}

pub struct AttackEffects {
    timer: Timer,
    flash_speed: f32,
    screen_shake_amount: f32,
    current_shake: f32,
}
pub struct BattlePlugin;

pub struct FightEvent {
    target: Entity,
    damage_amount: isize,
    next_state: BattleState,
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
            .add_state(BattleState::PlayerTurn)
            .insert_resource(AttackEffects {
                timer: Timer::from_seconds(0.7, true),
                flash_speed: 0.1,
                current_shake: 0.0,
                screen_shake_amount: 0.1,
            })
            .insert_resource(BattleMenuSelection {
                selected: BattleMenuOption::Fight,
            })
            .add_system_set(
                SystemSet::on_update(BattleState::EnemyTurn(false)).with_system(process_enemy_turn),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Battle)
                    .with_system(battle_input)
                    .with_system(battle_camera)
                    .with_system(highlight_battle_buttons)
                    .with_system(damage_calculation),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Battle)
                    .with_system(set_starting_state)
                    .with_system(spawn_enemy)
                    .with_system(spawn_battle_menu),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Battle)
                    .with_system(despawn_enemy)
                    .with_system(despawn_menu),
            )
            .add_system_set(
                SystemSet::on_update(BattleState::PlayerAttack).with_system(handle_attack_effects),
            )
            .add_system_set(
                SystemSet::on_update(BattleState::EnemyAttack).with_system(handle_attack_effects),
            );
    }
}

fn handle_attack_effects(
    mut attack_fx: ResMut<AttackEffects>,
    time: Res<Time>,
    mut enemy_graphics_query: Query<&mut Visibility, With<Enemy>>,
    mut state: ResMut<State<BattleState>>,
) {
    attack_fx.timer.tick(time.delta());
    let mut enemy_sprite = enemy_graphics_query.iter_mut().next().unwrap();

    if state.current() == &BattleState::PlayerAttack {
        if attack_fx.timer.elapsed_secs() % attack_fx.flash_speed > attack_fx.flash_speed / 2.0 {
            enemy_sprite.is_visible = false;
        } else {
            enemy_sprite.is_visible = true;
        }
    } else {
        attack_fx.current_shake = attack_fx.screen_shake_amount
            * f32::sin(attack_fx.timer.percent() * 2.0 * std::f32::consts::PI);
    }

    if attack_fx.timer.just_finished() {
        enemy_sprite.is_visible = true;
        if state.current() == &BattleState::PlayerAttack {
            state.set(BattleState::EnemyTurn(false)).unwrap();
        } else {
            state.set(BattleState::PlayerTurn).unwrap();
        }
    }
}

fn set_starting_state(mut state: ResMut<State<BattleState>>) {
    let _ = state.set(BattleState::PlayerTurn);
}

const NUM_MENU_OPTIONS: isize = 2;
#[derive(Component, PartialEq, Clone, Copy)]
pub enum BattleMenuOption {
    Fight,
    Run,
}

pub struct BattleMenuSelection {
    selected: BattleMenuOption,
}

fn spawn_battle_button(
    commands: &mut Commands,
    ascii: &AsciiSpriteSheet,
    indices: &NineSliceIndices,
    translation: Vec3,
    text: &str,
    id: BattleMenuOption,
    size: Vec2,
) -> Entity {
    let fight_nine_slice = spawn_nine_slice(commands, ascii, indices, size.x, size.y);

    let x_offset = (-size.x / 2.0 + 1.5) * TILE_SIZE;
    let fight_text = spawn_ascii_text(commands, ascii, text, Vec3::new(x_offset, 0.0, 0.0));

    commands
        .spawn_bundle(SpatialBundle::default())
        .insert(Transform {
            translation: translation,
            ..Default::default()
        })
        .insert(Name::new("Button"))
        .insert(id)
        .add_child(fight_text)
        .add_child(fight_nine_slice)
        .id()
}

fn spawn_battle_menu(
    mut commands: Commands,
    ascii: Res<AsciiSpriteSheet>,
    nine_slice_indices: Res<NineSliceIndices>,
) {
    let box_height = 3.0;
    let box_center_y = -1.0 + box_height * TILE_SIZE / 2.0;

    let run_text = "Run";
    let run_width = (run_text.len() + 2) as f32;
    let run_center_x = RESOLUTION - (run_width * TILE_SIZE) / 2.0;

    spawn_battle_button(
        &mut commands,
        &ascii,
        &nine_slice_indices,
        Vec3::new(run_center_x, box_center_y, 100.0),
        run_text,
        BattleMenuOption::Run,
        Vec2::new(run_width, box_height),
    );

    let fight_text = "Fight";
    let fight_width = (fight_text.len() + 2) as f32;
    let fight_center_x = RESOLUTION - (run_width * TILE_SIZE) - (fight_width * TILE_SIZE / 2.0);

    spawn_battle_button(
        &mut commands,
        &ascii,
        &nine_slice_indices,
        Vec3::new(fight_center_x, box_center_y, 100.0),
        fight_text,
        BattleMenuOption::Fight,
        Vec2::new(fight_width, box_height),
    );
}

fn process_enemy_turn(
    mut fight_event: EventWriter<FightEvent>,
    mut battle_state: ResMut<State<BattleState>>,
    enemy_query: Query<&BattleStats, With<Enemy>>,
    player_query: Query<Entity, With<Player>>,
) {
    let player_ent = player_query.single();
    let enemy_stats = enemy_query.iter().next().unwrap();
    fight_event.send(FightEvent {
        target: player_ent,
        damage_amount: enemy_stats.attack,
        next_state: BattleState::EnemyAttack,
    });
    battle_state.set(BattleState::EnemyTurn(true)).unwrap();
}

fn despawn_menu(mut commands: Commands, button_query: Query<Entity, With<BattleMenuOption>>) {
    for button in button_query.iter() {
        commands.entity(button).despawn_recursive();
    }
}

fn damage_calculation(
    mut commands: Commands,
    ascii: Res<AsciiSpriteSheet>,
    mut fight_event: EventReader<FightEvent>,
    text_query: Query<&AsciiText>,
    mut target_query: Query<(&Children, &mut BattleStats)>,
    mut battle_state: ResMut<State<BattleState>>,
) {
    if let Some(fight_event) = fight_event.iter().next() {
        //Get target stats and children
        let (target_children, mut stats) = target_query
            .get_mut(fight_event.target)
            .expect("Fighting enemy without stats");

        //Damage calc
        stats.health = std::cmp::max(
            stats.health - (fight_event.damage_amount - stats.defense),
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
                    &format!("Health: {}", stats.health as usize),
                    //relative to enemy pos
                    Vec3::new(-4.5 * TILE_SIZE, 2.0 * TILE_SIZE, 100.0),
                );

                commands.entity(fight_event.target).add_child(new_health);
            }
        }

        //Kill enemy if dead
        //TODO support multiple enemies
        //FIXME should create fadeout!
        if stats.health == 0 {
            create_fadeout(&mut commands, GameState::Overworld, &ascii);
            battle_state.set(BattleState::Exiting).unwrap();
        } else {
            battle_state.set(fight_event.next_state).unwrap();
        }
    }
}

fn battle_input(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    player_query: Query<&BattleStats, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut fight_event: EventWriter<FightEvent>,
    mut menu_state: ResMut<BattleMenuSelection>,
    mut battle_state: ResMut<State<BattleState>>,
    ascii: Res<AsciiSpriteSheet>,
) {
    if battle_state.current() != &BattleState::PlayerTurn {
        return;
    }

    let player_battle = player_query.single();

    //TODO handle multiple enemies
    let enemy = enemy_query.single();
    let mut new_selection = menu_state.selected as isize;
    if keyboard.just_pressed(KeyCode::A) {
        new_selection -= 1;
    }
    if keyboard.just_pressed(KeyCode::D) {
        new_selection += 1;
    }
    new_selection = (new_selection + NUM_MENU_OPTIONS) % NUM_MENU_OPTIONS;

    menu_state.selected = match new_selection {
        0 => BattleMenuOption::Fight,
        1 => BattleMenuOption::Run,
        _ => unreachable!("Bad menu selection"),
    };

    if keyboard.just_pressed(KeyCode::Space) {
        match menu_state.selected {
            BattleMenuOption::Fight => fight_event.send(FightEvent {
                //TODO select enemy and attack type
                target: enemy,
                damage_amount: player_battle.attack,
                next_state: BattleState::PlayerAttack,
            }),
            BattleMenuOption::Run => {
                create_fadeout(&mut commands, GameState::Overworld, &ascii);
                battle_state.set(BattleState::Exiting).unwrap()
            }
        }
    }
}

fn battle_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    attack_fx: Res<AttackEffects>,
) {
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = attack_fx.current_shake;
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
        Vec3::splat(1.0),
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

fn highlight_battle_buttons(
    menu_state: Res<BattleMenuSelection>,
    button_query: Query<(&Children, &BattleMenuOption)>,
    nine_slice_query: Query<&Children, With<NineSlice>>,
    mut sprites_query: Query<&mut TextureAtlasSprite>,
) {
    for (button_children, button_id) in button_query.iter() {
        for button_child in button_children.iter() {
            if let Ok(nine_slice_children) = nine_slice_query.get(*button_child) {
                for nine_slice_child in nine_slice_children.iter() {
                    if let Ok(mut sprite) = sprites_query.get_mut(*nine_slice_child) {
                        if menu_state.selected == *button_id {
                            sprite.color = Color::RED;
                        } else {
                            sprite.color = Color::WHITE;
                        }
                    }
                }
            }
        }
    }
}
