use bevy::app::App;
use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;

pub struct PhysicsConfPlugin;

impl Plugin for PhysicsConfPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, disable_grav);
    }
}

fn disable_grav(mut conf:ResMut<RapierConfiguration>) {
    conf.gravity = Vec2::ZERO;
}