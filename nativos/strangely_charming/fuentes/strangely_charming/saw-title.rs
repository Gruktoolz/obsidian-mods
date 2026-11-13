fn hook(
    value: horizon_world::nbt::Tag,
    game: &crate::game::Game<'_>,
    station: &Station,
) -> horizon_world::nbt::Tag {
    if game.block_name(station.pos).as_ref() == "strangely_charming:woodcutter" {
        horizon_protocol::text::literal("Table Saw")
    } else {
        value
    }
}
