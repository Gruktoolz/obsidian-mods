fn flair_window(game: &mut crate::game::Game<'_>, event: &winit::event::WindowEvent) -> core::ops::ControlFlow<()> {
    crate::game::umbx_flair_input::window(game, event)
}
