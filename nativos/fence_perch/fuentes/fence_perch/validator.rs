// Original prototype orchestration; no Horizon dependency in this module.
pub type Bounds = [f64; 6];
pub type Verdict = (bool, bool, bool);

/// Bound extra state lookups independently of the native solver's own budget.
pub const PREFLIGHT_BUDGET: usize = 4096;

#[derive(Clone, Copy, Debug)]
pub struct Request {
    pub old: Bounds,
    pub new: Bounds,
    pub movement: [f64; 3],
    pub creative: bool,
    pub enabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cells {
    pub min: [i32; 3],
    pub max: [i32; 3],
    pub count: usize,
}

/// Conservative neighboring cells for ordinary barrier geometry and stepping.
pub fn preflight_cells(request: &Request) -> Option<Cells> {
    if request.movement.iter().any(|v| !v.is_finite()) {
        return None;
    }
    for bounds in [request.old, request.new] {
        if bounds.iter().any(|v| !v.is_finite())
            || (0..3).any(|axis| bounds[axis] >= bounds[axis + 3])
        {
            return None;
        }
    }
    let mut cells = Cells { min: [0; 3], max: [0; 3], count: 1 };
    for axis in 0..3 {
        let lo = request.old[axis].min(request.new[axis]).floor() - 1.0;
        let extra_step = if axis == 1 { 0.6_f32 as f64 } else { 0.0 };
        let hi = (request.old[axis + 3].max(request.new[axis + 3]) + extra_step)
            .floor() + 1.0;
        if lo < f64::from(i32::MIN) || hi > f64::from(i32::MAX) {
            return None;
        }
        cells.min[axis] = lo as i32;
        cells.max[axis] = hi as i32;
        let length = usize::try_from(i64::from(cells.max[axis]) - i64::from(cells.min[axis]) + 1).ok()?;
        cells.count = cells.count.checked_mul(length)?;
        if cells.count > PREFLIGHT_BUDGET {
            return None;
        }
    }
    Some(cells)
}

pub trait Geometry {
    fn eligible_nearby(&mut self, cells: Cells) -> Result<bool, ()>;
    fn resolve(&mut self, request: &Request) -> Result<[f64; 3], ()>;
    fn changed(&self) -> bool;
    fn old_clear(&mut self, bounds: Bounds) -> Result<bool, ()>;
    fn new_collision(&mut self, old: Bounds, new: Bounds) -> Result<bool, ()>;
}

/// Preserve the upstream result outside the handled scope or on any probe error.
pub fn adapt<G: Geometry>(original: Verdict, request: Request, world: &mut G) -> Verdict {
    if !request.enabled {
        return original;
    }
    let Some(cells) = preflight_cells(&request) else { return original };
    if world.eligible_nearby(cells) != Ok(true) {
        return original;
    }
    let Ok(resolved) = world.resolve(&request) else { return original };
    if !world.changed() || resolved.iter().any(|v| !v.is_finite()) {
        return original;
    }
    let dx = request.movement[0] - resolved[0];
    let dz = request.movement[2] - resolved[2];
    let wrongly = !request.creative && dx * dx + dz * dz > 0.0625;
    let stands = request.movement[1] < 0.0 && resolved[1] != request.movement[1];
    if wrongly {
        match world.old_clear(request.old) {
            Ok(true) => return (true, true, stands),
            Ok(false) => {}
            Err(()) => return original,
        }
    }
    let shrink = f64::from(1.0e-5_f32);
    let deflate = |mut bounds: Bounds| {
        for axis in 0..3 {
            bounds[axis] += shrink;
            bounds[axis + 3] -= shrink;
        }
        bounds
    };
    match world.new_collision(deflate(request.old), deflate(request.new)) {
        Ok(refused) => (wrongly, refused, stands),
        Err(()) => original,
    }
}
