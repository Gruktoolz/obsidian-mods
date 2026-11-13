fn hook(m: &Mob, meta: &mut Vec<(u8, i32, Value)>) {
    if matches!(
        m.kind.class_id(),
        "minecraft:villager" | "minecraft:zombie_villager"
    ) {
        meta.push((
            120,
            horizon_protocol::entity_data::serializer::INT,
            Value::Int((i32::from(m.extra.variant3) << 8) | i32::from(m.extra.variant2)),
        ));
    }
}
