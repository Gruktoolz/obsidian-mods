// Player-scoped collision provider shared by client movement and server validation.
// Shared world geometry and mob queries are untouched.

use crate::fence_perch_core;
use horizon_core::{
    geometry::{Aabb, ShapeOp, Vec3, VoxelShape},
    position::{BlockPos, ChunkPos},
};
use horizon_world::{
    block_collision_scan::{BlockCollisionWorld, CollisionBlock},
    collision_context::CollisionContext,
    entity_obstruction::EntityIdentity,
    registry::BlockRegistry,
    world_movement::EntityMovementWorld,
};
use std::io;

pub(crate) struct PlayerQuery<'r, W> {
    pub inner: W,
    pub registry: &'r BlockRegistry,
    /// Set once any supported barrier shape was actually capped.
    pub changed: bool,
}

impl<'r, W> PlayerQuery<'r, W> {
    pub fn new(inner: W, registry: &'r BlockRegistry) -> Self {
        Self { inner, registry, changed: false }
    }
}

impl<W: BlockCollisionWorld<Error = io::Error>> BlockCollisionWorld for PlayerQuery<'_, W> {
    type Error = io::Error;
    type Chunk = W::Chunk;

    fn collision_chunk(&mut self, pos: ChunkPos) -> io::Result<Option<Self::Chunk>> {
        self.inner.collision_chunk(pos)
    }

    fn collision_block(&mut self, chunk: Self::Chunk, pos: BlockPos) -> io::Result<CollisionBlock> {
        self.inner.collision_block(chunk, pos)
    }

    fn is_suffocating(
        &mut self,
        chunk: Self::Chunk,
        block: CollisionBlock,
        pos: BlockPos,
    ) -> io::Result<bool> {
        self.inner.is_suffocating(chunk, block, pos)
    }

    fn context_shape(
        &mut self,
        block: CollisionBlock,
        pos: BlockPos,
        context: &CollisionContext<'_>,
    ) -> io::Result<VoxelShape> {
        let original = self.inner.context_shape(block, pos, context)?;
        let Some(state) = self.registry.state(block.state) else {
            return Ok(original);
        };
        if !fence_perch_core::supports(
            &state.name,
            state.properties.get("open").map(String::as_str),
        ) {
            return Ok(original);
        }
        let Some(bounds) = original.bounds() else {
            return Ok(original);
        };
        let lo = bounds.min_position();
        let hi = bounds.max_position();
        if hi.y <= fence_perch_core::PERCH_HEIGHT {
            return Ok(original);
        }
        let clipped = match fence_perch_core::capped_bounds([lo.x, lo.y, lo.z, hi.x, hi.y, hi.z]) {
            Ok(Some(bounds)) => bounds,
            Ok(None) => {
                self.changed = true;
                return Ok(VoxelShape::empty());
            }
            Err(_) => return Ok(original),
        };
        let limit = Aabb::new(
            Vec3::new(clipped[0], clipped[1], clipped[2]),
            Vec3::new(clipped[3], clipped[4], clipped[5]),
        );
        // On an unexpected shape error, preserve the solid original geometry.
        let Ok(limit) = VoxelShape::from_box(limit) else {
            return Ok(original);
        };
        let Ok(capped) = original.join(&limit, ShapeOp::AND) else {
            return Ok(original);
        };
        self.changed = true;
        Ok(capped)
    }

    fn may_reach_out(&mut self, chunk: Self::Chunk, section_y: i32) -> io::Result<bool> {
        self.inner.may_reach_out(chunk, section_y)
    }

    fn section_is_empty(&mut self, chunk: Self::Chunk, section_y: i32) -> io::Result<bool> {
        self.inner.section_is_empty(chunk, section_y)
    }

    fn missing_chunks_stay_missing(&self) -> bool {
        self.inner.missing_chunks_stay_missing()
    }
}

impl<W: EntityMovementWorld<Error = io::Error>> EntityMovementWorld for PlayerQuery<'_, W> {
    type Error = io::Error;

    fn entity_collisions(
        &mut self,
        entity: EntityIdentity,
        bounds: Aabb,
    ) -> io::Result<Vec<VoxelShape>> {
        self.inner.entity_collisions(entity, bounds)
    }

    fn movement_border(
        &mut self,
        entity: EntityIdentity,
        bounds: Aabb,
    ) -> io::Result<Option<VoxelShape>> {
        self.inner.movement_border(entity, bounds)
    }
}
