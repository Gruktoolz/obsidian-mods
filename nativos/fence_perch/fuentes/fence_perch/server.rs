// Server collision verdict for players near supported barriers.
use super::{Hub, TickCtx};
use crate::{
    collisions::SceneCollisions,
    fence_perch_core,
    fence_perch_query::PlayerQuery,
    fence_perch_validation::{self as policy, Bounds, Cells, Geometry, Request, Verdict},
};
use horizon_core::geometry::{Aabb, ShapeOp, Vec3, VoxelShape};
use horizon_world::{
    block_collision_scan::BlockCollisionScan,
    collision_context::CollisionContext,
    entity_obstruction::EntityIdentity,
    world_movement::{self, CollisionCheckRequest, MovementRequest},
};

struct NativeGeometry<'w, 'r> {
    query: PlayerQuery<'r, SceneCollisions<'w, 'r>>,
    context: CollisionContext<'static>,
    on_ground: bool,
}

fn body(bounds: Bounds) -> Aabb {
    Aabb::new(Vec3::new(bounds[0], bounds[1], bounds[2]), Vec3::new(bounds[3], bounds[4], bounds[5]))
}

impl Geometry for NativeGeometry<'_, '_> {
    fn eligible_nearby(&mut self, cells: Cells) -> Result<bool, ()> {
        for x in cells.min[0]..=cells.max[0] {
            for y in cells.min[1]..=cells.max[1] {
                for z in cells.min[2]..=cells.max[2] {
                    let id = self.query.inner.world.block(x, y, z);
                    let Some(state) = self.query.registry.state(id) else { return Err(()) };
                    if fence_perch_core::supports(&state.name, state.properties.get("open").map(String::as_str)) {
                        return Ok(true);
                    }
                }
            }
        }
        Ok(false)
    }

    fn resolve(&mut self, request: &Request) -> Result<[f64; 3], ()> {
        let [x, y, z] = request.movement;
        let resolved = world_movement::resolve_world_movement(&mut self.query, MovementRequest {
            entity: EntityIdentity(0), context: self.context, bounds: body(request.old),
            movement: Vec3::new(x, y, z), max_step: 0.6, on_ground: self.on_ground,
            candidate_budget: 1_000_000,
        }).map_err(|_| ())?;
        Ok([resolved.x, resolved.y, resolved.z])
    }

    fn changed(&self) -> bool { self.query.changed }

    fn old_clear(&mut self, bounds: Bounds) -> Result<bool, ()> {
        world_movement::no_world_collision(&mut self.query, CollisionCheckRequest {
            entity: EntityIdentity(0), context: CollisionContext::position(bounds[1]),
            bounds: body(bounds), candidate_budget: 1_000_000,
        }).map_err(|_| ())
    }

    fn new_collision(&mut self, old: Bounds, new: Bounds) -> Result<bool, ()> {
        let previous = VoxelShape::from_box(body(old)).map_err(|_| ())?;
        let scan = BlockCollisionScan::new(&mut self.query, self.context, body(new), false, 1 << 20)
            .map_err(|_| ())?;
        for hit in scan {
            let hit = hit.map_err(|_| ())?;
            if !hit.shape.join_is_not_empty(&previous, ShapeOp::AND).map_err(|_| ())? {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

pub(super) fn adapt(original: Verdict, hub: &Hub, index: usize, destination: [f64; 3], ctx: &TickCtx<'_>) -> Verdict {
    let Some(binding) = ctx.collision else { return original };
    let Some(player) = hub.guests.get(index) else { return original };
    if player.spectator() || player.sleeping { return original; }
    let dimensions = player.dimensions();
    let radius = f64::from(dimensions.width) / 2.0;
    let height = f64::from(dimensions.height);
    let bounds = |position: [f64; 3]| -> Bounds {
        [position[0] - radius, position[1], position[2] - radius,
         position[0] + radius, position[1] + height, position[2] + radius]
    };
    let request = Request {
        old: bounds(player.position), new: bounds(destination),
        movement: [0, 1, 2].map(|axis| destination[axis] - player.position[axis]),
        creative: player.creative(), enabled: true,
    };
    let mut world = NativeGeometry {
        query: PlayerQuery::new(
            SceneCollisions { world: &*ctx.world, binding, subject: None },
            binding.registry(),
        ),
        context: CollisionContext::position(player.position[1]).with_descending(player.sneaking),
        on_ground: player.on_ground,
    };
    policy::adapt(original, request, &mut world)
}
