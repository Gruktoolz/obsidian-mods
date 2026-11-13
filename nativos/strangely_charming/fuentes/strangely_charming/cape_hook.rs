fn hook(m: &horizon_client::mobs::Mob) -> core::ops::ControlFlow<()> {
    if m.skin
        .as_deref()
        .and_then(horizon_client_render::strangely_charming_skin::get)
        .is_some_and(|s| s.features.cape)
    {
        return core::ops::ControlFlow::Break(());
    }
    core::ops::ControlFlow::Continue(())
}
