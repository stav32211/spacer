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
    let player = &mut commands
        .spawn(PlayerBundle::default());

    attach_child_force_emitter(player, 150., 5.);
}

pub fn emitter_test(mut commands: Commands) {
    let force_center = &mut commands.spawn(SpatialBundle::default());

    attach_child_force_emitter(force_center, 150., 5.);
}