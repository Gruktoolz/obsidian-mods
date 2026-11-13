// ChatHeads' half in the chat's own crate: the chat's lines move right by
// the head and its gap while the mod is on, and the lines that get a head are
// recorded here for the game half to draw. A static, because the chat has no
// `Game` to ask; `draw_chat`'s HEAD clears it each frame, before the paint.

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

/// What the chat's lines move right by: a head is 8 GUI pixels and keeps 2
/// of gap to its text (`runtime.rs`'s `HEAD` and the gap there).
const INDENT: i32 = 10;

pub static ON: AtomicBool = AtomicBool::new(false);

/// A line that gets a head: where it was laid out, the opacity it is painted
/// with, and its plain text.
pub type Head = (i32, i32, u8, String);

static LINES: Mutex<Vec<Head>> = Mutex::new(Vec::new());

/// Called before each chat paint: a frame that paints no line shows no head.
pub fn clear() {
    LINES.lock().unwrap_or_else(|e| e.into_inner()).clear();
}

/// The lines the last paint recorded, in paint order.
pub fn lines() -> Vec<Head> {
    LINES.lock().unwrap_or_else(|e| e.into_inner()).clone()
}

pub fn shifted(mut draws: Vec<crate::hud_layout::GuiDraw>) -> Vec<crate::hud_layout::GuiDraw> {
    if !ON.load(Ordering::Relaxed) {
        return draws;
    }
    let mut heads = Vec::new();
    for draw in &mut draws {
        // The chat's own lines are the text it draws at x = 0.
        let crate::hud_layout::GuiDraw::Text {
            text, x, y, color, ..
        } = draw
        else {
            continue;
        };
        if *x != 0 {
            continue;
        }
        let plain = text.plain();
        if plain.starts_with('<') {
            heads.push((*x, *y, (*color >> 24) as u8, plain));
        }
        *x = INDENT;
    }
    *LINES.lock().unwrap_or_else(|e| e.into_inner()) = heads;
    draws
}
