// use bevy::ecs::query::QueryEntityError;
// use bevy::ecs::system::EntityCommands;
// use bevy::math::Vec2;
// use bevy::prelude::*;
// use bevy_debug_text_overlay::screen_print;
// use bevy_rapier2d::prelude::*;
// use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
// use crate::game::components::force_emitter::ForceEmitter;
// use crate::game::GameSet;
// 
// 
// #[derive(Event, Copy, Clone, Debug, PartialEq, Eq)]
// struct GravityZoneCollision {
//     zone: Entity,
//     object: Entity,
//     collision_type: GravityZoneCollisionType,
// }
// 
// #[derive(Copy, Clone, Debug, PartialEq, Eq)]
// enum GravityZoneCollisionType {
//     ENTER,
//     EXIT,
// }
// 
// impl From<&CollisionEvent> for GravityZoneCollisionType {
//     fn from(value: &CollisionEvent) -> Self {
//         match value {
//             CollisionEvent::Started(_, _, _) => { GravityZoneCollisionType::ENTER }
//             CollisionEvent::Stopped(_, _, _) => { GravityZoneCollisionType::EXIT }
//         }
//     }
// }
// 
// impl GravityZoneCollision {
//     fn try_from_collision<'w, 'a>(
//         //a: &Entity,
//         //b: &Entity,
//         event: &CollisionEvent,
//         gravity_centers: &Query<'w, 'a, Entity, With<ForceEmitter>>,
//         external_force: &Query<'w, 'a, Entity, With<ExternalForce>>,
//     ) -> Option<GravityZoneCollision> {
//         match event {
//             CollisionEvent::Started(a, b, flags) |
//             CollisionEvent::Stopped(a, b, flags) => {
//                 if flags.contains(CollisionEventFlags::SENSOR) &&
//                     gravity_centers.contains(*a) && external_force.contains(*b) {
//                     Some(GravityZoneCollision {
//                         object: *b,
//                         zone: *a,
//                         collision_type: GravityZoneCollisionType::from(event),
//                     })
//                 } else { None }
//             }
//         }
//     }
// }