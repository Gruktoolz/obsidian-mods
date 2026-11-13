fn kotost_chest_press(game: &mut crate::game::Game<'_>) -> std::ops::ControlFlow<bool> {
    if crate::game::tweaks::kotost_chest::press(game) {
        return std::ops::ControlFlow::Break(true);
    }
    std::ops::ControlFlow::Continue(())
}
