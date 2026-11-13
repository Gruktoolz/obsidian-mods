fn hook(profession: &str, level: i32) -> core::ops::ControlFlow<String> {
    if crate::mobs::villager::evanders_jobs::number(profession).is_some() {
        return core::ops::ControlFlow::Break(format!("{profession}/level_{level}"));
    }
    core::ops::ControlFlow::Continue(())
}
