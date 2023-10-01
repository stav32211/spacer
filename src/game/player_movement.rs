use bevy::math::vec2;
use bevy::prelude::*;
use bevy_debug_text_overlay::screen_print;
use bevy_rapier2d::dynamics::ExternalImpulse;
use bevy_rapier2d::prelude::{ExternalForce, Velocity};
use super::components::player::*;
use super::GameSet;

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (move_player_wasd, player_dashing).chain().in_set(GameSet::Main));
    }
}

fn move_player_wasd(
    mut player_query: Query<(&mut ExternalForce, &mut Player, &mut Velocity)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>) {
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

    let delta_time = time.delta_seconds();
    let normalized_movement = movement_input.normalize_or_zero();
    let force_delta = normalized_movement * delta_time * 1000. * PLAYER_SPEED;

    let (mut ext_force, mut player, mut velocity) = player_query.single_mut();
    let speed = velocity.linvel.length();

    if speed < 0.5 {
        velocity.linvel = Vec2::ZERO; // TODO sleep instead?
    }

    screen_print!("speed: {speed}");
    screen_print!("delta time: {delta_time}");
    let delta = (force_delta);

    if normalized_movement != Vec2::ZERO {
        player.last_movement_direction = normalized_movement;
    }
    let new_force = ext_force.force + delta;
    let len = new_force.length();
    screen_print!("{len}");

    screen_print!("update");
    ext_force.force = new_force;

    let weakened_force = ext_force.force * 0.95_f32.powf(delta_time * 60.);
    ext_force.force = if weakened_force.length() < 5. {
        Vec2::ZERO
    } else {
        weakened_force
    }
}

fn player_dashing(mut player_query: Query<(&mut ExternalImpulse, &Player)>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        let (mut ext_imp, player) = player_query.single_mut();
        let dash = player.last_movement_direction * PLAYER_DASH_AMOUNT;
        ext_imp.impulse += dash;
    }
}