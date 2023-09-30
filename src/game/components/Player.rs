use bevy::prelude::Component;
use bevy::math::Vec2;
pub const PLAYER_RADIUS: f32 = 20.;
pub const PLAYER_SPEED: f32 = 160.;
pub const PLAYER_DASH_AMOUNT: f32 = 500.;
#[derive(Component, Default,Copy, Clone)]
pub struct Player {
    pub last_movement_direction: Vec2,
}
