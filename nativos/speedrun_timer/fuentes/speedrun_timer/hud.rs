// The timer's box in its corner: HH:MM:SS.mmm, gold while it runs, green once
// it has stopped, grey while it waits, with the personal best beside it.
fn speedrun_timer_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::speedrun_timer as m;
    use horizon_client::extras::Corner;
    // Turned off in Options > Mods: nothing until the next build leaves it out.
    if !game.native_on(m::ID) {
        return;
    }
    let settings = {
        let game = &*game;
        m::settings(|key| game.native_setting(m::ID, key))
    };
    if !settings.shown {
        return;
    }
    let corner = [
        Corner::TopLeft,
        Corner::TopRight,
        Corner::BottomLeft,
        Corner::BottomRight,
    ][settings.corner];
    // The debug screen takes the top left: leave that corner to it.
    if !game.native_corner_shown(corner) {
        return;
    }
    let (phase, nanos) = m::reading(std::time::Instant::now());
    // No world yet: no box at all, so the title screen stays clean.
    if phase == m::Phase::Idle {
        return;
    }
    let text = match phase {
        m::Phase::Done => format!("{}  done", m::format(nanos)),
        _ => m::format(nanos),
    };
    let text = if settings.best && settings.best_ms > 0 {
        format!("{text}   PB {}", m::format(settings.best_ms * 1_000_000))
    } else {
        text
    };
    let ink = match phase {
        m::Phase::Running => m::GOLD,
        m::Phase::Done => m::GREEN,
        _ => m::DIM,
    };
    let w = game.native_text_width(&text) + 4.0;
    let (x, y) = corners.place(corner, w, m::ROW_H);
    game.native_fill(batch, [x, y, w, m::ROW_H], m::BACKGROUND);
    game.native_text(batch, &text, [x + 2.0, y + 2.0], ink, true);
}
