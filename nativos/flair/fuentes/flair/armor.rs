use std::ops::ControlFlow;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

static HIDE: AtomicBool = AtomicBool::new(false);
static LOCAL: AtomicU32 = AtomicU32::new(u32::MAX);

pub fn set(hide: bool, local: Option<u32>) {
    HIDE.store(hide, Ordering::Relaxed);
    LOCAL.store(local.unwrap_or(u32::MAX), Ordering::Relaxed);
}

#[allow(clippy::too_many_arguments)]
pub fn hidden(
    m: &super::Mob,
    alpha: f32,
    camera: glam::DVec3,
    atlas: super::Atlas<'_>,
    light: u32,
    data: &super::MobData,
    extra: &[(&super::Part, [f32; 4], u32, u32, bool)],
    out: &mut Vec<crate::mesher::Vertex>,
) -> ControlFlow<()> {
    if extra.is_empty() || !HIDE.load(Ordering::Relaxed) || m.id != LOCAL.load(Ordering::Relaxed) {
        return ControlFlow::Continue(());
    }
    super::render_with(m, alpha, camera, atlas, light, data, &[], out);
    ControlFlow::Break(())
}
