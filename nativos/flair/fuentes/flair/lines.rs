use super::Game;
use super::umbx_flair_catalog as catalog;
use super::umbx_flair_glue as glue;
use super::umbx_flair_items::Slot;
use horizon_client::gizmos::Line;
use std::cell::Cell;
use std::ops::ControlFlow;

const REFERENCE_WIDTH: f32 = 1920.0;

thread_local! {
    static DRAWING: Cell<bool> = const { Cell::new(false) };
}

pub(super) fn lines(game: &Game<'_>, alpha: f64, size: [u32; 2]) -> ControlFlow<Vec<Line>> {
    if DRAWING.get() || !game.native_on(catalog::ID) || game.own_invisible() {
        return ControlFlow::Continue(());
    }
    let Some(m) = game.player_model.as_ref() else { return ControlFlow::Continue(()) };
    let yaw = m.previous_body_yaw + (m.body_yaw - m.previous_body_yaw) * alpha as f32;
    let mut strokes = Vec::new();
    {
        let flair = glue::state();
        let shown = game.player_model_shown() || flair.first_person[Slot::Aura as usize];
        if let Some(i) = flair.worn(Slot::Aura).filter(|_| shown) {
            flair.entries[i].item.stroke(f64::from(flair.seconds()), f64::from(yaw), flair.theme, &mut strokes);
        }
    }
    if strokes.is_empty() {
        return ControlFlow::Continue(());
    }
    DRAWING.set(true);
    let mut lines = game.fishing_line(alpha, size);
    DRAWING.set(false);
    let lerp = |a: f64, b: f64| a + (b - a) * alpha;
    let feet = [lerp(m.previous.x, m.position.x), lerp(m.previous.y, m.position.y), lerp(m.previous.z, m.position.z)];
    let scale = (size[0] as f32 / REFERENCE_WIDTH).max(1.0);
    let at = |p: [f64; 3]| std::array::from_fn(|i| feet[i] + p[i]);
    lines.extend(strokes.iter().map(|s| Line { start: at(s.a), end: at(s.b), color: s.color, width: s.width * scale }));
    ControlFlow::Break(lines)
}
