// `held_item_geometry`: with the vanilla hand, the model's own hands hold
// nothing (the hand already shows the items).
fn first_person_body_skip_held() -> core::ops::ControlFlow<()> {
    use horizon_client::mobs::first_person_body as m;
    if m::active().is_some() && m::current().get(m::o::ARMS) == 0 {
        return core::ops::ControlFlow::Break(());
    }
    core::ops::ControlFlow::Continue(())
}
