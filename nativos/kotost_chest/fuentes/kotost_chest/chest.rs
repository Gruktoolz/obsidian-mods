// Kotost chest: beside a chest-like container's sort button (Horizon's inventory extras), two more:
// "everything into the chest" (the main inventory, hotbar kept) and "everything out of the chest". Each is a run
// of ordinary Shift-clicks (`Input::QuickMove`), so a server takes them as vanilla's.
// Lives in crate::game::tweaks, so it reaches the sort buttons' layout and the extras' clicks.

use super::{Game, HudBatch, Input, SORT_BUTTON, rgba};

/// The two buttons: where (screen corner, from the inventory's origin), what they move, their tooltip.
struct Button {
    x: f32,
    y: f32,
    into_chest: bool,
}

/// Slots of the open container (300..500), and the buttons left of its sort button. None when no chest-like
/// container is open or the sort buttons are off.
fn buttons(game: &Game<'_>) -> Option<(Vec<usize>, [Button; 2])> {
    let (corner, slots) = game
        .sort_buttons()
        .into_iter()
        .find(|(_, slots)| slots.first().is_some_and(|s| (300..500).contains(s)))?;
    let (x, y) = corner;
    let step = SORT_BUTTON + 2.0;
    Some((
        slots,
        [
            Button { x: x - step, y, into_chest: false },
            Button { x: x - 2.0 * step, y, into_chest: true },
        ],
    ))
}

fn over(game: &Game<'_>, b: &Button, ox: f32, oy: f32) -> bool {
    let (cx, cy) = game.cursor;
    let (x, y) = (ox + b.x, oy + b.y);
    cx >= x && cx < x + SORT_BUTTON && cy >= y && cy < y + SORT_BUTTON
}

/// `draw_sort_buttons`' head: the two buttons, an arrow each (up: into the chest, down: out of it).
pub fn draw(game: &Game<'_>, batch: &mut HudBatch, ox: f32, oy: f32) {
    let Some((_, buttons)) = buttons(game) else {
        return;
    };
    let atlas = &game.gui.atlas;
    let ink = rgba(55, 55, 55, 255);
    let mut tip = None;
    for b in &buttons {
        let hovered = over(game, b, ox, oy);
        let face = if hovered { rgba(220, 220, 220, 255) } else { rgba(160, 160, 160, 255) };
        let (x, y) = (ox + b.x, oy + b.y);
        batch.fill(atlas, x, y, SORT_BUTTON, SORT_BUTTON, ink);
        batch.fill(atlas, x + 1.0, y + 1.0, SORT_BUTTON - 2.0, SORT_BUTTON - 2.0, face);
        // An arrow: a stem and a head of three rows (5, 3, 1 pixels wide).
        let (head, stem) = if b.into_chest { (2.0, 5.0) } else { (4.0, 2.0) };
        batch.fill(atlas, x + 4.0, y + stem, 1.0, 2.0, ink);
        for (i, w) in [1.0f32, 3.0, 5.0].into_iter().enumerate() {
            let row = if b.into_chest { head + i as f32 } else { head + 2.0 - i as f32 };
            batch.fill(atlas, x + 4.5 - w / 2.0, y + row, w, 1.0, ink);
        }
        if hovered {
            tip = Some((if b.into_chest { "Всё в сундук" } else { "Всё из сундука" }, x, y));
        }
    }
    if let Some((text, x, y)) = tip {
        let w = game.native_text_width(text);
        let (tx, ty) = (x + SORT_BUTTON / 2.0 - w / 2.0, y - 12.0);
        batch.fill(atlas, tx - 3.0, ty - 2.0, w + 6.0, 12.0, rgba(16, 0, 16, 230));
        game.native_text(batch, text, [tx, ty], 0xFFFFFF, true);
    }
}

/// `sort_press`' head: a left press on one of the buttons moves the items (true: the press is taken).
pub fn press(game: &mut Game<'_>) -> bool {
    if game.carried.is_some() {
        return false;
    }
    let Some((slots, buttons)) = buttons(game) else {
        return false;
    };
    let Some(r) = game.renderer.as_ref() else {
        return false;
    };
    let batch = HudBatch::new([r.config.width, r.config.height]);
    let (ox, oy) = game.inventory_origin(&batch);
    let Some(b) = buttons.iter().find(|b| over(game, b, ox, oy)) else {
        return false;
    };
    // Into the chest: the main inventory (9..36), the hotbar stays; out of it: every container slot.
    let from: Vec<usize> = if b.into_chest { (9..36).collect() } else { slots };
    for slot in from {
        if game.slot_view(slot).is_some() {
            game.tweak_click(slot, 0, Input::QuickMove);
        }
    }
    true
}
