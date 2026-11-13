fn deepbore_quarry_bounding_box(
    entity: &horizon_client::containers::ModBlockEntity,
    bounds: &mut [f32; 6],
) {
    crate::game::deepbore_quarry_glue::bounds(entity, bounds);
}
