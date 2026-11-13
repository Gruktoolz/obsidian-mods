fn hook(
    m: &crate::mobs::Mob,
    alpha: f32,
    base: glam::Mat4,
    atlas: crate::mobs::render::Atlas<'_>,
    light: u32,
    data: &crate::mobs::MobData,
    body_visible: bool,
    out: &mut Vec<crate::mesher::Vertex>,
) {
    crate::mobs::strangely_charming_geometry::draw(
        m,
        alpha,
        base,
        atlas,
        light,
        data,
        body_visible,
        out,
    );
}
