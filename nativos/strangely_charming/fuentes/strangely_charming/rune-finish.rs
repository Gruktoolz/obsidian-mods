fn hook(
    game: &mut crate::game::Game<'_>,
    host: &mut LanHost,
    conn: u32,
    pos: [i32; 3],
) -> core::ops::ControlFlow<()> {
    if !game.sc_rune_at(pos) {
        return core::ops::ControlFlow::Continue(());
    }
    let per_tick = game
        .as_guest(host, conn, |g| g.destroy_progress(pos))
        .unwrap_or(0.0);
    let ticks = host.hub.ticks;
    if let Some(play) = host.play.get_mut(&conn) {
        let start = play.dig_start_for(pos);
        play.dig_ended();
        play.destroy_pos = None;
        play.delayed = None;
        if let Some(start) = start {
            let progress = (ticks.saturating_sub(start) + 1) as f32 * per_tick;
            if progress >= 1.0 {
                game.guest_destroy(host, conn, pos);
            } else if progress >= 0.7 {
                // Tolerate normal client/server tick skew, but only break after the full interval.
                play.delayed = Some((pos, start));
            }
        }
    }
    core::ops::ControlFlow::Break(())
}
