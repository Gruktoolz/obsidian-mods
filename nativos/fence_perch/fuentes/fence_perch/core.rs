// Dependency-free barrier selection and block-local collision clipping.

/// The maximum collision height for an eligible player query, in blocks.
pub const PERCH_HEIGHT: f64 = 1.0;

/// Select vanilla barrier families. A gate must explicitly report closed.
pub fn supports(name: &str, open: Option<&str>) -> bool {
    let Some(path) = name.strip_prefix("minecraft:") else {
        return false;
    };
    if path.ends_with("_fence_gate") {
        return open == Some("false");
    }
    path.ends_with("_fence") || path.ends_with("_wall")
}

/// A malformed bounding box must never turn a solid shape into empty space.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidBounds;

/// Clip only the upper Y bound. None means no volume remains below the cap.
/// X/Z coordinates and the lower Y bound are never expanded or quantized.
pub fn capped_bounds(bounds: [f64; 6]) -> Result<Option<[f64; 6]>, InvalidBounds> {
    capped_bounds_at(bounds, PERCH_HEIGHT)
}

/// Clip bounds in their own coordinate system, including world-space scan hits.
pub fn capped_bounds_at(
    bounds: [f64; 6],
    cap: f64,
) -> Result<Option<[f64; 6]>, InvalidBounds> {
    if !cap.is_finite()
        || bounds.iter().any(|v| !v.is_finite())
        || (0..3).any(|axis| bounds[axis] >= bounds[axis + 3])
    {
        return Err(InvalidBounds);
    }
    let mut clipped = bounds;
    clipped[4] = clipped[4].min(cap);
    Ok((clipped[1] < clipped[4]).then_some(clipped))
}

/// A capped hit can refuse movement only if it still touches the new body.
/// Unknown geometry preserves the caller's original refusal decision.
/// Kept for the standalone policy tests; the in-game server path uses `validator.rs`.
#[allow(dead_code)]
pub fn refusal_after_cap(
    original: bool,
    overlaps_new: Option<bool>,
    overlaps_old: Option<bool>,
) -> bool {
    match overlaps_new {
        Some(false) => false,
        Some(true) => overlaps_old.map_or(original, |overlap| !overlap),
        None => original,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_fences_walls_and_closed_gates() {
        for name in [
            "minecraft:oak_fence",
            "minecraft:nether_brick_fence",
            "minecraft:pale_oak_fence",
            "minecraft:cobblestone_wall",
            "minecraft:polished_deepslate_wall",
        ] {
            assert!(supports(name, None), "{name}");
        }
        assert!(supports("minecraft:oak_fence_gate", Some("false")));
    }

    #[test]
    fn leaves_open_or_unknown_gate_state_unchanged() {
        for state in [None, Some("true"), Some("False"), Some("unknown")] {
            assert!(!supports("minecraft:oak_fence_gate", state));
        }
    }

    #[test]
    fn rejects_other_blocks_and_namespaces() {
        for name in [
            "minecraft:stone",
            "minecraft:iron_bars",
            "minecraft:oak_wall_sign",
            "minecraft:wall_torch",
            "example:oak_fence",
            "example:cobblestone_wall",
            "oak_fence",
            "",
        ] {
            assert!(!supports(name, Some("false")), "{name}");
        }
    }

    #[test]
    fn preserves_thin_horizontal_geometry_and_base() {
        assert_eq!(
            capped_bounds([0.375, 0.3125, 0.0, 0.625, 1.5, 1.0]),
            Ok(Some([0.375, 0.3125, 0.0, 0.625, 1.0, 1.0]))
        );
    }

    #[test]
    fn leaves_short_shapes_unchanged() {
        let bounds = [0.0, 0.0, 0.0, 1.0, 0.75, 1.0];
        assert_eq!(capped_bounds(bounds), Ok(Some(bounds)));
    }

    #[test]
    fn clips_each_connected_part_without_filling_gaps() {
        let post = [0.375, 0.0, 0.375, 0.625, 1.5, 0.625];
        let rail = [0.4375, 0.375, 0.0, 0.5625, 1.5, 0.375];
        let a = capped_bounds(post).unwrap().unwrap();
        let b = capped_bounds(rail).unwrap().unwrap();
        assert_eq!([a[0], a[2], a[3], a[5]], [0.375, 0.375, 0.625, 0.625]);
        assert_eq!([b[0], b[2], b[3], b[5]], [0.4375, 0.0, 0.5625, 0.375]);
        assert_eq!((a[4], b[4]), (1.0, 1.0));
    }

    #[test]
    fn removes_only_volume_above_cap() {
        assert_eq!(capped_bounds([0.0, 1.0, 0.0, 1.0, 1.5, 1.0]), Ok(None));
    }

    #[test]
    fn rejects_invalid_geometry_instead_of_removing_it() {
        for bounds in [
            [0.0, 0.0, 0.0, 1.0, f64::NAN, 1.0],
            [0.0, 0.0, 0.0, f64::INFINITY, 1.5, 1.0],
            [0.5, 0.0, 0.0, 0.25, 1.5, 1.0],
            [0.0, 1.0, 0.0, 1.0, 1.0, 1.0],
        ] {
            assert_eq!(capped_bounds(bounds), Err(InvalidBounds));
        }
    }
}
