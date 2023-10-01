use std::collections::HashMap;

use bevy::ecs::system::EntityCommands;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;

pub struct ForceEmittingPlugin;

impl Plugin for ForceEmittingPlugin {
    fn build(&self, app: &mut App) {
        
        app.add_systems(PreUpdate, update_force_emitter_on_sensor_collision);
    }
}


#[derive(Component, Clone)]
pub struct ForceEmitter {
    intensity: f32,
    owner: Entity,
    force_handles: HashMap<Entity, Vec2>,
}

pub fn attach_child_force_emitter<'w, 's, 'a>(
    entity_commands: &'a mut EntityCommands<'w, 's, 'a>,
    radius: f32,
    intensity: f32) -> &'a mut EntityCommands<'w, 's, 'a> {
    let owner = entity_commands.id();

    entity_commands.with_children(|parent| {
        parent
            //.spawn(RigidBody::Fixed)
            .spawn(Collider::ball(radius))
            .insert(Transform::default())
            .insert(Sensor)
            //.insert(ActiveCollisionTypes::DYNAMIC_STATIC | ActiveCollisionTypes::KINEMATIC_STATIC)
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(ColliderMassProperties::Mass(0.0))
            .insert(ForceEmitter {
                intensity,
                owner,
                force_handles: HashMap::new(),
            });
    })
}

fn collision_is_gravity_zone<'w, 'a>(
    event: &CollisionEvent,
    gravity_centers: &'w Query<'w, 'a, (Entity, &mut ForceEmitter)>,
    external_force: &Query<'w, 'a, Entity, (With<ExternalForce>, Without<ForceEmitter>)>) -> Option<(Entity, Entity)> {
    match event {
        CollisionEvent::Started(a, b, flags) |
        CollisionEvent::Stopped(a, b, flags) => {
            if !flags.contains(CollisionEventFlags::SENSOR) { return None; }

            let a_emitter = gravity_centers.contains(*a);
            let b_receiver = external_force.contains(*b);

            let b_emitter = gravity_centers.contains(*b);
            let a_receiver = external_force.contains(*a);

            if a_emitter && b_receiver { Some((*a, *b)) } else if b_emitter && a_receiver { Some((*b, *a)) } else { None }
        }
    }
}

fn update_force_emitter_on_sensor_collision(
    mut events: EventReader<CollisionEvent>,
    mut gravity_centers: Query<(Entity, &mut ForceEmitter)>,
    external_force: Query<Entity, (With<ExternalForce>, Without<ForceEmitter>)>,
) {
    for event in events.iter() {
        let is_gravity_collision = collision_is_gravity_zone(event, &gravity_centers, &external_force);

        let (center_entity, other_entity) = if is_gravity_collision.is_some() { is_gravity_collision.unwrap() } else { continue; };
        let mut center = gravity_centers.get_mut(center_entity).unwrap().1;
        let other = external_force.get(other_entity).unwrap();

        match event {
            CollisionEvent::Started(_, _, _) => {
                center.force_handles.insert(other, Vec2::ZERO);
                println!("in");
            }
            CollisionEvent::Stopped(_, _, _) => {
                center.force_handles.remove(&other);
                println!("out");
            }
        }
    }
}
