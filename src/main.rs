mod game;

use std::time::Duration;

use bevy::asset::ChangeWatcher;
use bevy::prelude::*;
use game::GamePlugin;


fn main() {
    println!("Hello, world!");

    let watcher = ChangeWatcher::with_delay(Duration::from_millis(3000));
    let asset_plugin = AssetPlugin {
        watch_for_changes: watcher,
        ..default()
    };

    App::new()
        // base
        .add_plugins(DefaultPlugins.set(asset_plugin))
        .add_plugins(GamePlugin)
        .run();
}




