// Client movement adapter: routes the local player's queries through the shared provider.

use super::MotionWorld;
use crate::fence_perch_query::PlayerQuery;
use horizon_core::geometry::{Aabb, Vec3};
use horizon_world::{
    entity_obstruction::EntityIdentity,
    ground_support::find_supporting_block,
    world_movement::{self, CollisionCheckRequest, MovementRequest},
};

pub(super) fn collide(motion: &mut MotionWorld<'_, '_, '_>, delta: Vec3) -> crate::Result<Vec3> {
    let max_step = motion.player.attribute("minecraft:step_height")? as f32;
    let request = MovementRequest {
        entity: EntityIdentity(0),
        context: motion.context(),
        bounds: motion.player.bounds(),
        movement: delta,
        max_step,
        on_ground: motion.player.ground.on_ground,
        candidate_budget: 1_000_000,
    };
    let mut query = PlayerQuery::new(motion.movement_collisions(), motion.assets.registry);
    Ok(world_movement::resolve_world_movement(&mut query, request)?)
}

pub(super) fn clear(motion: &mut MotionWorld<'_, '_, '_>, bounds: Aabb) -> crate::Result<bool> {
    let request = CollisionCheckRequest {
        entity: EntityIdentity(0),
        context: motion.context(),
        bounds,
        candidate_budget: 1_000_000,
    };
    let mut query = PlayerQuery::new(motion.movement_collisions(), motion.assets.registry);
    Ok(world_movement::no_world_collision(&mut query, request)?)
}

pub(super) fn ground(
    motion: &mut MotionWorld<'_, '_, '_>,
    below: bool,
    horizontal: bool,
    movement: Vec3,
) -> crate::Result<()> {
    let context = motion.context();
    let position = motion.player.position;
    let bounds = motion.player.bounds();
    let inner = super::SceneCollisions {
        world: &*motion.scene,
        binding: motion.collision_binding,
        subject: Some(motion.player.powder_snow_subject()),
    };
    let mut query = PlayerQuery::new(inner, motion.assets.registry);
    motion.player.ground.set_with_movement(
        below,
        horizontal,
        bounds,
        Some(movement),
        |area| find_supporting_block(&mut query, context, position, area, 1_000_000),
    )?;
    Ok(())
}
