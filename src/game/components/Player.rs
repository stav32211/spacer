use bevy::prelude::{Bundle, Component, Sprite, Transform};
use bevy::math::Vec2;
use bevy::sprite::SpriteBundle;
use bevy::utils::default;
use bevy_rapier2d::dynamics::{Damping, ExternalForce, Velocity};
use bevy_rapier2d::prelude::{Collider, ColliderMassProperties, ExternalImpulse, RigidBody};

pub const PLAYER_RADIUS: f32 = 20.;
pub const PLAYER_SPEED: f32 = 40.;
pub const PLAYER_DASH_AMOUNT: f32 = 800.;
pub const PLAYER_DAMPING: f32 = 25.;

#[derive(Component, Default, Copy, Clone)]
pub struct Player {
    pub last_movement_direction: Vec2,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    collider: Collider,
    player: Player,
    collider_mass_properties: ColliderMassProperties,
    rigid_body: RigidBody,
    velocity: Velocity,
    external_impulse: ExternalImpulse,
    external_force: ExternalForce,
    damping: Damping,
    sprite_bundle: SpriteBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        let sprite_bundle = SpriteBundle {
            sprite: Sprite {
                color: Default::default(),
                flip_x: false,
                flip_y: false,
                custom_size: None,
                rect: None,
                anchor: Default::default(),
            },
            transform: Transform::from_xyz(0.0, 0.0, 00.0),
            ..default()
        };

        Self {
            collider: Collider::ball(PLAYER_RADIUS),
            damping: Damping { linear_damping: PLAYER_DAMPING, ..default() },
            external_force: ExternalForce::default(),
            external_impulse: ExternalImpulse::default(),
            collider_mass_properties: ColliderMassProperties::Density(1.),
            rigid_body: RigidBody::default(),
            player: Player::default(),
            velocity: Default::default(),
            sprite_bundle
        }
    }
}
