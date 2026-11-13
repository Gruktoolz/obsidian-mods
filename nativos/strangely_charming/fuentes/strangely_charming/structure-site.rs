fn hook(
    checks: &StartChecks,
    queries: &mut impl CheckQueries,
    chunk: ChunkPos,
    y: i32,
    biomes: &BTreeSet<BiomeId>,
) -> core::ops::ControlFlow<bool> {
    use core::ops::ControlFlow::{Break, Continue};
    let Some(mode) = checks.max_y else {
        return Continue(());
    };
    if mode != i32::MAX - 1 && mode != i32::MAX - 2 {
        return Continue(());
    }
    let (x, z) = (chunk.x * 16, chunk.z * 16);
    // Reject other biome families before sampling up to nine full noise columns.
    let sample_y = if mode == i32::MAX - 1 { queries.sea_level() } else { y };
    if !biomes.contains(&queries.noise_biome([x >> 2, sample_y >> 2, z >> 2])) {
        return Break(false);
    }
    let bottom = queries.bottom();
    let mut low = i32::MAX;
    let mut high = i32::MIN;
    // The start can rotate into any quadrant around the chunk corner.
    for dx in [-12, 0, 12] {
        for dz in [-12, 0, 12] {
            let (filled, solid) = queries.liquid_column(x + dx, z + dz);
            if mode == i32::MAX - 2 {
                // A buried floor and ceiling, including in the Nether above its lava sea.
                if [y - 2, y + 10]
                    .iter()
                    .any(|h| solid.get((h - bottom) as usize) != Some(&true))
                {
                    return Break(false);
                }
            } else {
                let terrain = solid.iter().rposition(|&b| b);
                if terrain.is_none() || terrain != filled.iter().rposition(|&b| b) {
                    return Break(false);
                }
                let h = terrain.unwrap() as i32;
                low = low.min(h);
                high = high.max(h);
                if high - low > 2 {
                    return Break(false);
                }
            }
        }
    }
    Continue(())
}
