use bevy::app::Startup;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::math::vec2;
use bevy_debug_text_overlay::screen_print;
use super::{utils, components::player::*};
use super::GameSet;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, add_camera)
            .add_systems(Update, camera_follow_player.in_set(GameSet::CameraMovement));
    }
}

pub fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn camera_follow_player(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut camera_q: Query<(&GlobalTransform, &Camera, &mut Transform)>,
    player_transform_q: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut gizmos: Gizmos,
) {
    // TODO work with viewport center and distance from it
    let window = window_q.single();
    let (camera_global_pos, camera, mut camera_pos) = camera_q.single_mut();
    let viewport = utils::camera_viewport(camera, camera_global_pos, window);

    let Some(viewport) = viewport else { return; };

    // screen coords
    gizmos.circle_2d(viewport.min, 10., Color::WHITE);
    gizmos.circle_2d(viewport.max, 10., Color::WHITE);

    let width_limit = viewport.width() * 0.23;
    let height_limit = viewport.height() * 0.23;

    // calc limits
    let left_limit = viewport.min.x + width_limit;
    let right_limit = viewport.max.x - width_limit;
    let up_limit = viewport.min.y + height_limit;
    let down_limit = viewport.max.y - height_limit;

    // draw lines
    gizmos.line_2d(vec2(left_limit, 5000.), vec2(left_limit, -5000.), Color::RED);
    gizmos.line_2d(vec2(right_limit, 5000.), vec2(right_limit, -5000.), Color::RED);
    gizmos.line_2d(vec2(5000., up_limit), vec2(-5000., up_limit), Color::PINK);
    gizmos.line_2d(vec2(5000., down_limit), vec2(-5000., down_limit), Color::PINK);

    let player_translation = player_transform_q.single().translation;

    let left_dif = left_limit - player_translation.x;
    let right_dif = right_limit - player_translation.x;
    let up_dif = up_limit - player_translation.y;
    let down_dif = down_limit - player_translation.y;

    if right_dif < 0. {
        camera_pos.translation.x -= right_dif / 5. ;
        screen_print!("move right!");
    }
    if left_dif > 0. {
        camera_pos.translation.x -= left_dif / 5.;
        screen_print!("move left!");
    }
    if up_dif < 0. {
        camera_pos.translation.y -= up_dif / 5.;
    }
    if down_dif > 0. {
        camera_pos.translation.y -= down_dif / 5.;
    }
}

