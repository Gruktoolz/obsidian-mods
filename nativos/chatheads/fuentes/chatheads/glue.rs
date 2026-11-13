// ChatHeads' game half: the mod's one setting, its page in Options > Mods,
// and the heads left of the chat's lines. `runtime.rs` holds the logic.

use std::cell::RefCell;
use std::sync::atomic::Ordering;

use horizon_client::hud::{HudAtlas, HudBatch, Rect};

use crate::game::chat::native_chatheads as m;
use crate::game::Game;
use horizon_client::chat::native_chatheads_layout as layout;

struct State {
    /// Read once, then only what a click writes.
    settings: Option<m::Settings>,
}

thread_local! {
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
}

fn with<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|cell| {
        f(cell
            .borrow_mut()
            .get_or_insert_with(|| State { settings: None }))
    })
}

/// The settings the player kept, read the first time.
pub(crate) fn settings(game: &Game<'_>) -> m::Settings {
    with(|s| {
        *s.settings
            .get_or_insert_with(|| m::Settings::load(|key| game.native_setting(m::ID, key)))
    })
}

/// A click on the mod's settings page: the next value, kept.
pub(crate) fn click(game: &Game<'_>, button: usize) {
    let mut settings = settings(game);
    settings.click(button);
    for (key, value) in settings.saved() {
        game.native_set_setting(m::ID, key, &value);
    }
    with(|s| s.settings = Some(settings));
}

/// Head of `draw_chat`: the chat's lines move right for the heads
/// (`shift.rs` reads this), and they are drawn once the chat is painted.
pub(crate) fn mark(game: &Game<'_>) {
    layout::ON.store(
        game.native_on(m::ID) && settings(game).heads,
        Ordering::Relaxed,
    );
    // A frame that paints no line must not keep the last frame's heads.
    layout::clear();
}

/// Tail of `draw_chat`: a player's head left of each line they wrote, over
/// the chat's own background. `shift.rs` recorded the lines it moved right,
/// so this asks the paint it already did rather than laying the chat out
/// again.
pub(crate) fn heads(game: &Game<'_>, batch: &mut HudBatch) {
    if !layout::ON.load(Ordering::Relaxed) {
        return;
    }
    let scale = game.ui.chat.options.scale as f32;
    for (x, y, alpha, line) in layout::lines() {
        let Some(name) = m::sender(&line) else {
            continue;
        };
        let Some(face) = face(&game.gui.atlas, name) else {
            continue;
        };
        let (x, y, side) = m::head(x, y, scale);
        // The line's own opacity, so a head fades out with its line.
        batch.rect_uv(
            x,
            y,
            side,
            side,
            face,
            horizon_client::hud::rgba(255, 255, 255, alpha),
        );
    }
}

/// The face of `name`'s skin: the skin the game resolved for that profile
/// (`atlas.object_skins`, as inline head components use it), else one of the
/// default skins. None when the atlas has neither.
fn face(atlas: &HudAtlas, name: &str) -> Option<Rect> {
    let key = format!("name:{}", name.to_ascii_lowercase());
    let skin = match atlas.object_skins.get(&key) {
        Some(path) if atlas.sprites.contains_key(path) => path.as_str(),
        _ => horizon_client::tab_list::default_skin(m::seed(name)),
    };
    let skin = *atlas.sprites.get(skin)?;
    let (du, dv) = ((skin.u1 - skin.u0) / m::SKIN, (skin.v1 - skin.v0) / m::SKIN);
    Some(Rect {
        u0: skin.u0 + du * m::FACE,
        v0: skin.v0 + dv * m::FACE,
        u1: skin.u0 + du * (m::FACE + m::HEAD),
        v1: skin.v0 + dv * (m::FACE + m::HEAD),
        w: m::HEAD as u32,
        h: m::HEAD as u32,
    })
}
