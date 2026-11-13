fn pose(
    value: glam::Mat4,
    game: &crate::game::Game<'_>,
    state: &horizon_client::item_render::ItemRenderState,
) -> glam::Mat4 {
    crate::game::fast_items_renderer::pose(value, game, state)
}
