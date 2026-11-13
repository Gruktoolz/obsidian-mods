fn events(
    game: &crate::game::Game<'_>,
    id: winit::window::WindowId,
    event: &winit::event::WindowEvent,
) {
    crate::game::dynamic_fps_adapter::event(game, id, event);
}
