fn hook(m: &Mob) -> core::ops::ControlFlow<Tag> {
    if let Some(id) = crate::mobs::villager::evanders_jobs::id(m.extra.variant2) {
        let value = Tag::compound([
            (
                "type",
                string(&format!(
                    "minecraft:{}",
                    crate::mobs::render::VILLAGER_TYPES
                        [m.extra.variant as usize % crate::mobs::render::VILLAGER_TYPES.len()]
                )),
            ),
            ("level", Tag::Int(i32::from(m.extra.variant3.max(1)))),
            ("profession", string(id)),
        ]);
        return core::ops::ControlFlow::Break(value);
    }
    core::ops::ControlFlow::Continue(())
}
