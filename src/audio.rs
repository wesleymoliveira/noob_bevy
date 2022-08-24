use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{battle::BattleState, battle::FightEvent, GameState};

pub struct GameAudioPlugin;

pub struct AudioState {
    bgm_handle: Handle<AudioSource>,
    battle_handle: Handle<AudioSource>,
    hit_handle: Handle<AudioSource>,
    reward_handle: Handle<AudioSource>,

    volume: f64,
}
#[derive(Component, Default, Clone)]
pub struct BackgroundChannel;

#[derive(Component, Default, Clone)]
pub struct BattleChannel;

#[derive(Component, Default, Clone)]
pub struct SfxChannel;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_audio)
            .add_audio_channel::<BackgroundChannel>()
            .add_audio_channel::<BattleChannel>()
            .add_audio_channel::<SfxChannel>()
            .add_system_set(SystemSet::on_enter(GameState::Battle).with_system(start_battle_music))
            .add_system_set(SystemSet::on_enter(GameState::Overworld).with_system(resume_bgm_music))
            .add_system_set(SystemSet::on_enter(BattleState::Reward).with_system(play_reward_sfx))
            .add_system(play_hit_sfx)
            .add_system(volume_control)
            .add_startup_system(start_bgm_music);
    }
}

fn load_audio(
    mut commands: Commands,
    background: Res<AudioChannel<BackgroundChannel>>,
    battle: Res<AudioChannel<BattleChannel>>,
    sfx: Res<AudioChannel<SfxChannel>>,
    assets: Res<AssetServer>,
) {
    let bgm_handle = assets.load("bip-bop.ogg");
    let battle_handle = assets.load("ganxta.ogg");
    let hit_handle = assets.load("hit.wav");
    let reward_handle = assets.load("reward.wav");

    let volume = 0.5;

    background.set_volume(volume);
    battle.set_volume(volume);
    sfx.set_volume(volume);

    commands.insert_resource(AudioState {
        bgm_handle: bgm_handle,
        battle_handle: battle_handle,
        hit_handle: hit_handle,
        reward_handle: reward_handle,
        volume,
    });
}

fn play_reward_sfx(audio_state: Res<AudioState>, sfx: Res<AudioChannel<SfxChannel>>) {
    sfx.play(audio_state.reward_handle.clone());
}

fn start_bgm_music(audio_state: Res<AudioState>, background: Res<AudioChannel<BackgroundChannel>>) {
    background.play(audio_state.bgm_handle.clone()).looped();
}

fn play_hit_sfx(
    sfx: Res<AudioChannel<SfxChannel>>,
    audio_state: Res<AudioState>,
    mut fight_event: EventReader<FightEvent>,
) {
    if fight_event.iter().count() > 0 {
        sfx.play(audio_state.hit_handle.clone());
    }
}

fn resume_bgm_music(
    background: Res<AudioChannel<BackgroundChannel>>,
    battle: Res<AudioChannel<BattleChannel>>,
) {
    battle.stop();
    background.resume();
}

fn start_battle_music(
    background: Res<AudioChannel<BackgroundChannel>>,
    battle: Res<AudioChannel<BattleChannel>>,
    audio_state: Res<AudioState>,
) {
    background.pause();
    battle.play(audio_state.battle_handle.clone()).looped();
}

fn volume_control(
    keyboard: Res<Input<KeyCode>>,
    background: Res<AudioChannel<BackgroundChannel>>,
    battle: Res<AudioChannel<BattleChannel>>,
    sfx: Res<AudioChannel<SfxChannel>>,
    mut audio_state: ResMut<AudioState>,
) {
    if keyboard.just_pressed(KeyCode::Up) {
        audio_state.volume += 0.10;
    }
    if keyboard.just_pressed(KeyCode::Down) {
        audio_state.volume -= 0.10;
    }
    audio_state.volume = audio_state.volume.clamp(0.0, 1.0);

    background.set_volume(audio_state.volume);
    battle.set_volume(audio_state.volume);
    sfx.set_volume(audio_state.volume);
}
