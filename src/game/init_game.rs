use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::components::player::PLAYER_RADIUS;

use super::components::{force_emitter::*, player::*};

pub struct InitGamePlugin;

impl Plugin for InitGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                spawn_player,
                emitter_test
            ))
            //.add_systems(PostUpdate, display_intersection_info)
            .insert_resource(ClearColor(BG_COLOR));
    }
}

const BG_COLOR: Color = Color::Rgba {
    alpha: 1.,
    red: 9. / 256.,
    green: 17. / 256.,
    blue: 41. / 256.,
};

pub fn spawn_player(mut commands: Commands) {
    let sprite = SpriteBundle {
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

    let player = &mut commands
        .spawn(Collider::ball(PLAYER_RADIUS));

    player.insert(ColliderMassProperties::Density(1.0))
        .insert(sprite)
        .insert(GravityScale(0.0)) // TODO global conf
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(Player::default())
        .insert(ExternalImpulse::default())
        .insert(ExternalForce::default())
        .insert(Damping { linear_damping: 3., ..default() })
    ;

    attach_child_force_emitter(player, 150., 5.);
}

pub fn emitter_test(mut commands: Commands) {
    let force_center = &mut commands.spawn(SpatialBundle::default());

    attach_child_force_emitter(force_center, 150., 5.);
}