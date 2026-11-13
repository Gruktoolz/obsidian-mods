fn hook(entity: &EntityType) -> core::ops::ControlFlow<bool> {
    if matches!(
        (entity.id.as_str(), entity.template.as_str()),
        ("strangely_charming:coral_squid", "minecraft:squid")
    ) {
        core::ops::ControlFlow::Break(true)
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
