use bevy::prelude::{Camera, GlobalTransform, Window};
use bevy::math::{Rect, Vec2, vec2};

pub fn camera_viewport(camera: &Camera, camera_global_pos: &GlobalTransform, window: &Window) -> Option<Rect> {
    let (width, height) = (window.width(), window.height());

    let up_left_corner =
        camera.viewport_to_world_2d(camera_global_pos, Vec2::ZERO)?;

    let down_right_corner =
        camera.viewport_to_world_2d(camera_global_pos, vec2(width, height))?;

    Some(Rect {
        min: up_left_corner,
        max: down_right_corner,
    })
}
