fn hook(m: &Mob, alpha: f32, body: f32) -> core::ops::ControlFlow<Option<glam::Mat4>> {
    if m.kind.id != "strangely_charming:coral_squid" {
        return core::ops::ControlFlow::Continue(());
    }
    let x = m.anim.lerp(X_BODY_ROT, alpha);
    let z = m.anim.lerp(Z_BODY_ROT, alpha);
    let centre = glam::Mat4::from_translation(glam::Vec3::Y * (m.kind.height as f32 * 0.5));
    // LivingEntityRenderer's model baseline is 1.501; our head pivot is 16 model pixels down.
    let pivot = glam::Mat4::from_translation(-glam::Vec3::Y * (1.501 - 16.0 / 16.0));
    core::ops::ControlFlow::Break(Some(
        centre
            * glam::Mat4::from_rotation_y((180. - body).to_radians())
            * glam::Mat4::from_rotation_x(x.to_radians())
            * glam::Mat4::from_rotation_y(z.to_radians())
            * pivot,
    ))
}
