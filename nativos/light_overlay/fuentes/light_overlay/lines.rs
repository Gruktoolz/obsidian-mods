// Hook helper: fishing_line (cancellable HEAD)
fn light_overlay_lines(
    game: &crate::game::Game<'_>,
    alpha: f64,
    size: [u32; 2],
) -> core::ops::ControlFlow<Vec<Line>> {
    if !crate::game::mod_light_overlay_glue::should_render_overlay(game) {
        return core::ops::ControlFlow::Continue(());
    }
    let own = game
        .hook_position(alpha)
        .zip(game.rod_hand_position(alpha, size));
    let width = horizon_client::gfx::outline_width(size[0]);
    let mut lines = Vec::new();
    for (hook, hand) in own.into_iter().chain(game.remote_hook_ends(alpha)) {
        let offset = (hand - (hook + glam::DVec3::new(0.0, 0.25, 0.0))).as_vec3();
        let point = |a: f32| {
            let y = offset.y * (a * a + a) * 0.5 + 0.25;
            hook + glam::Vec3::new(offset.x * a, y, offset.z * a).as_dvec3()
        };
        lines.extend((0..16).map(|i| Line {
            start: point(i as f32 / 16.0).to_array(),
            end: point((i + 1) as f32 / 16.0).to_array(),
            color: 0xFF00_0000,
            width,
        }));
    }
    crate::game::mod_light_overlay_glue::append_overlay_lines(game, &mut lines, width);
    core::ops::ControlFlow::Break(lines)
}
