// After setupRotations: the whole player leans, banks, bounces and squashes.
fn livingmotion_body(value: glam::Mat4, m: &crate::mobs::Mob, alpha: f32) -> glam::Mat4 {
    crate::mobs::livingmotion::body(m, alpha, value)
}
