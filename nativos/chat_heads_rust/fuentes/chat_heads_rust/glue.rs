// The mod's game half: the settings, their page, and the heads left of the
// chat's lines. `runtime.rs` holds the logic.

use std::cell::RefCell;
use std::sync::atomic::Ordering;

use horizon_client::hud::{HudAtlas, HudBatch, Rect};

use crate::game::chat::native_chat_heads_rust as m;
use crate::game::Game;
use horizon_client::chat::native_chat_heads_rust_layout as layout;

struct State {
    /// Read once, then only what a click writes.
    settings: Option<m::Settings>,
}

thread_local! {
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
}

fn with<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|cell| {
        f(cell.borrow_mut().get_or_insert_with(|| State { settings: None }))
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

/// Head of `draw_chat`: the chat's lines move right for the heads (`shift.rs`
/// reads this), and they are drawn once the chat is painted.
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
/// so this reads the paint the chat already did rather than laying it out
/// again.
pub(crate) fn heads(game: &mut Game<'_>, batch: &mut HudBatch) {
    if !layout::ON.load(Ordering::Relaxed) {
        return;
    }
    let settings = settings(game);
    let scale = game.ui.chat.options.scale as f32;
    // The player's own name, straight from the game (works for Microsoft
    // accounts and cracked players alike); `/chatheads me` overrides it.
    let me = game
        .native_setting(m::ID, "own_name")
        .filter(|n| !n.trim().is_empty())
        .unwrap_or_else(|| game.options.player_name());
    // The server's own player list: the resolver's last resort for lines in a
    // server's own format (ranks, nicks, plugins), and the names the atlas'
    // name-keyed skins were filled from. `remote_tab_players` also pushes the
    // downloaded skins into the atlas, so a line drawn before its player's
    // skin was ready shows the steady default that frame and the real face
    // the next one — nothing is skipped for good.
    let known: Vec<String> = game
        .remote_tab_players()
        .map(|(players, _, _)| players.into_iter().map(|p| p.name).collect())
        .unwrap_or_default();
    for (x, y, alpha, line) in layout::lines() {
        // Server lines carry `§` codes and their own formats: strip, then
        // match — the strict `<Name>`, a bare `Name:`/`Name >` start, else
        // the earliest tab-list name the line mentions.
        let stripped = m::strip_codes(&line);
        let Some((_, name)) = m::match_sender(&stripped, &known) else {
            continue;
        };
        // Your own lines (the server echoes yours in the same formats), when
        // the setting keeps them bare. Read at draw time, every frame.
        if !settings.own && name.eq_ignore_ascii_case(me.trim()) {
            continue;
        }
        let Some(layers) = face(&game.gui.atlas, name) else {
            continue; // never a crash, never a blank box: the line just stays bare
        };
        let (x, y, side) = m::head(x, y, scale);
        // The line's own opacity, so a head fades out with its line.
        let ink = horizon_client::hud::rgba(255, 255, 255, alpha);
        let [face, hat] = layers;
        batch.rect_uv(x, y, side, side, face, ink);
        // The hat layer over the face: hair, hats and glasses show.
        batch.rect_uv(x, y, side, side, hat, ink);
    }
}

/// A head's layers for `name`, resolved with no option and no Mojang API:
/// 1. the skin the game itself resolved for that player (`atlas.object_skins`,
///    keyed by name - the game fills it from the tab list's player info,
///    which Microsoft-account and offline-mode (cracked) servers send alike),
/// 2. else one of the default skins, picked steadily from the name, so a
///    player the game has no skin for keeps one identity,
/// 3. none: the caller draws no head at all.
/// Both skin layers come back - the face and the hat overlay.
fn face(atlas: &HudAtlas, name: &str) -> Option<[Rect; 2]> {
    let key = format!("name:{}", name.to_ascii_lowercase());
    let resolved = match atlas.object_skins.get(&key) {
        Some(path) if atlas.sprites.contains_key(path) => path.as_str(),
        _ => horizon_client::tab_list::default_skin(m::seed(name)),
    };
    let skin = *atlas.sprites.get(resolved)?;
    let (du, dv) = ((skin.u1 - skin.u0) / m::SKIN, (skin.v1 - skin.v0) / m::SKIN);
    Some(m::layers(skin.u0, skin.v0, du, dv).map(|(u0, v0, u1, v1)| Rect {
        u0,
        v0,
        u1,
        v1,
        w: m::HEAD as u32,
        h: m::HEAD as u32,
    }))
}
