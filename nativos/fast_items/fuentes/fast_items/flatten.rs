fn flatten(
    value: horizon_client::item_render::ItemRenderState,
) -> horizon_client::item_render::ItemRenderState {
    crate::game::fast_items_renderer::prepare(value)
}
