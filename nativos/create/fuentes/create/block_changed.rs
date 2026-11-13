fn create_changed(
    dimension: horizon_world::level::Dimension,
    pos: horizon_core::position::BlockPos,
) {
    let dimension = match dimension {
        horizon_world::level::Dimension::Overworld => "minecraft:overworld",
        horizon_world::level::Dimension::Nether => "minecraft:the_nether",
        horizon_world::level::Dimension::End => "minecraft:the_end",
    };
    let pos = [pos.x, pos.y, pos.z];
    crate::game::create_port_millstone::forget(pos);
    crate::game::create_port_fan::changed(dimension, pos);
    crate::game::create_port_particles::changed(dimension, pos);
    crate::game::create_port_drill::changed(dimension, pos);
    crate::game::create_port_kinetic_runtime::changed(dimension, pos);
}
