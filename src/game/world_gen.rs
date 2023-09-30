use bevy::math::*;
use bevy::prelude::*;
use bevy::prelude::{IntoSystemConfigs, Update};
use bevy::utils::HashMap;
use bevy_rapier2d::prelude::Vect;

use super::components::player::Player;
use super::GameSet;

const TILE_SIZE: f32 = 100.;


pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Map>()
            .add_systems(Update, (gen_tiles).in_set(GameSet::MapGen));
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TileLocation([i32; 2]);

impl TileLocation {
    fn all_in_radius(&self, radius: usize) -> Option<Vec<TileLocation>> {
        let radius = i32::try_from(radius).unwrap();
        let [x, y] = self.0;
        let range_x = x - radius..=x + radius;
        let range_y = y - radius..=y + radius;

        Some(range_x.flat_map(|x| {
            range_y.to_owned().map(move |y| {
                [x, y]
            })
        }).map(|coord| {
            TileLocation(coord)
        }).collect::<Vec<TileLocation>>())
    }

    fn as_vec(&self) -> Vec2 {
        vec2(self.0[0] as f32, self.0[1] as f32)
    }

    fn cord(&self) -> Vec2 {
        self.as_vec() * TILE_SIZE
    }

    fn center(&self) -> Vec2 {
        self.cord() + TILE_SIZE / 2.
    }
}

#[derive(Component, Clone, Default)]
pub struct MapTile {
    pub objects: Vec<Entity>,
}

#[derive(Resource, Default)]
pub struct Map {
    pub tiles: HashMap<TileLocation, MapTile>,
}

fn gen_tiles(mut map: ResMut<Map>, player_transform: Query<&Transform, With<Player>>, mut gizmos: Gizmos) {
    let player_location = player_transform.single().translation.truncate();
    let [player_x, player_y] = player_location.to_array();

    let tile_x = (player_x / TILE_SIZE).floor();
    let tile_y = (player_y / TILE_SIZE).floor();

    let tile_id = TileLocation([tile_x as i32, tile_y as i32]);

    let tiles_in_proximity =
        tile_id.all_in_radius(2).expect("too far");

    // drawing
    for tile in tiles_in_proximity.to_owned().iter_mut() {
        gizmos.rect_2d(
            tile.center(),
            0.,
            Vect::splat(TILE_SIZE),
            Color::WHITE,
        );
    };

    for coord in tiles_in_proximity.into_iter() {
        // TODO
        map.tiles.entry(coord).or_insert(MapTile::default());
    }
}