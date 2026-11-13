fn flair_armor(
    m: &Mob,
    alpha: f32,
    camera: glam::DVec3,
    atlas: Atlas<'_>,
    light: u32,
    data: &MobData,
    extra: &[(&Part, [f32; 4], u32, u32, bool)],
    out: &mut Vec<Vertex>,
) -> core::ops::ControlFlow<()> {
    crate::mobs::render::umbx_flair_armor::hidden(m, alpha, camera, atlas, light, data, extra, out)
}
