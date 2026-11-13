fn hook(m: &Mob, uuid: [i32; 4]) -> core::ops::ControlFlow<bool> {
    if m.extra.tame
        && m.extra.owner == Some(uuid)
        && horizon_mods::evanders_config::enabled("protect_pets")
    {
        core::ops::ControlFlow::Break(false)
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
