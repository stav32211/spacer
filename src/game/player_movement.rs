use bevy::math::vec2;
use bevy::prelude::*;
use bevy_debug_text_overlay::screen_print;
use bevy_rapier2d::dynamics::ExternalImpulse;
use bevy_rapier2d::prelude::Velocity;
use super::components::player::*;
use super::GameSet;

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (move_player_wasd, player_dashing).chain().in_set(GameSet::Main));
    }
}

fn move_player_wasd(mut player_query: Query<(&mut ExternalImpulse, &mut Player ,&mut Velocity)>, keyboard: Res<Input<KeyCode>>) {
    let mut movement_input = Vec2::ZERO;

    let up = vec2(0., 1.);
    let down = vec2(0., -1.);
    let right = vec2(1., 0.);
    let left = vec2(-1., 0.);

    if keyboard.pressed(KeyCode::W) {
        movement_input += up;
    }
    if keyboard.pressed(KeyCode::S) {
        movement_input += down;
    }
    if keyboard.pressed(KeyCode::D) {
        movement_input += right;
    }
    if keyboard.pressed(KeyCode::A) {
        movement_input += left;
    }

    let normalized_movement = movement_input.normalize_or_zero();
    let movement = normalized_movement * PLAYER_SPEED;

    let (mut ext_imp, mut player,mut velocity) = player_query.single_mut();
    let speed = velocity.linvel.length();
    if  speed < 0.5{
        velocity.linvel = Vec2::ZERO;
    }
    screen_print!("speed: {speed}");
    let speed_mod = speed.max(1.).sqrt();
    let delta =  movement / speed_mod;
    ext_imp.impulse +=delta;




    if normalized_movement != Vec2::ZERO {
        player.last_movement_direction = normalized_movement;
    }
}

fn player_dashing(mut player_query: Query<(&mut ExternalImpulse, &Player)>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        let (mut ext_imp, player) = player_query.single_mut();
        let dash = player.last_movement_direction * PLAYER_DASH_AMOUNT;
        ext_imp.impulse += dash;
    }
}