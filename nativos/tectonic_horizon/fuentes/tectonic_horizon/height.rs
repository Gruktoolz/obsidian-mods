fn height(
    overrides: &WorldgenOverrides,
    level: &str,
) -> std::ops::ControlFlow<Option<LevelHeight>> {
    #[allow(unused_imports)]
    use tectonic_selection::TectonicWorldSelection as _;

    let config = tectonic_horizon::config();
    if overrides.mod_enabled("tectonic_horizon") && config.general.mod_enabled
        && config.global_terrain.increased_height
        && level == "minecraft:overworld"
        && overrides.entries(DIMENSIONS).all(|(id, _)| id != level)
        && overrides
            .entries(DIMENSION_TYPES)
            .all(|(id, _)| id != level)
    {
        return std::ops::ControlFlow::Break(Some(LevelHeight {
            min_y: -64,
            height: 704,
            logical_height: 704,
            has_skylight: true,
            has_ceiling: false,
        }));
    }
    std::ops::ControlFlow::Continue(())
}
