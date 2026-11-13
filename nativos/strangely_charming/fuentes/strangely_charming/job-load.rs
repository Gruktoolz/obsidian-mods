fn hook(m: &mut Mob, tag: &Tag) -> core::ops::ControlFlow<()> {
    if let Some(prof) = tag
        .get("VillagerData")
        .and_then(|d| text(d.get("profession")))
        .as_deref()
        .and_then(crate::mobs::villager::evanders_jobs::number)
    {
        // Reuse the full vanilla load path, then restore the namespaced profession.
        let mut base = tag.clone();
        *base
            .get_mut("VillagerData")
            .unwrap()
            .get_mut("profession")
            .unwrap() = string("minecraft:fletcher");
        read_villager(m, &base);
        m.extra.variant2 = prof;
        return core::ops::ControlFlow::Break(());
    }
    core::ops::ControlFlow::Continue(())
}
