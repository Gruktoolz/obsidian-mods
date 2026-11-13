// Only MonumentBuilding constructs this 58 x 23 x 58 box at Y=39.
// Moving it here also moves all child frames derived by its constructor.
fn monuments(value: BoundingBox, y: i32, width: i32, height: i32, depth: i32) -> BoundingBox {
    let config = crate::worldgen::overrides::tectonic_horizon::config();
    if crate::worldgen::overrides::tectonic_horizon::active() && config.general.mod_enabled && (y, width, height, depth) == (39, 58, 23, 58) {
        return value.moved([0, config.oceans.monument_offset, 0]);
    }
    value
}
