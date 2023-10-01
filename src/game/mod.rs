mod components;
mod utils;
mod player_movement;
mod init_game;
mod camera;
mod world_gen;
mod events;
mod physics_conf;


use bevy::app::*;
use bevy::prelude::*;
use bevy_debug_text_overlay::{OverlayPlugin, screen_print};
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::prelude::RapierDebugRenderPlugin;
use crate::game::components::force_emitter::ForceEmittingPlugin;
use crate::game::physics_conf::PhysicsConfPlugin;
use crate::game::world_gen::WorldGenPlugin;
use self::init_game::*;
use self::player_movement::*;
use self::camera::*;


#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameSet {
    MapGen,
    Spawn,
    Main,
    CameraMovement,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
                            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0),
                            RapierDebugRenderPlugin::default()),
        )
            .add_plugins(OverlayPlugin { font_size: 23.0, ..default() })
            .configure_sets(Update, (GameSet::MapGen, GameSet::Spawn, GameSet::Main, GameSet::CameraMovement).chain())
            .add_plugins((
                CameraPlugin,
                InitGamePlugin,
                PlayerMovementPlugin,
                WorldGenPlugin,
                ForceEmittingPlugin,
                PhysicsConfPlugin
            ))
            .add_systems(PostUpdate, fps)
        ;
    }
}

fn fps(time: Res<Time>) {
    let fps = (1. / (time.delta().as_secs_f64())).round();
    screen_print!("{fps}")
}