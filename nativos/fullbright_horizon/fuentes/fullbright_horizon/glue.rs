// Glue between Horizon's `Game` and `mod_fullbright` runtime state.
use crate::game::Game;
use crate::game::mod_fullbright as m;
use crate::game::native_hooks::NativeMod;
use horizon_client::hud::HudBatch;
use std::time::Duration;

pub(super) fn on_key(game: &mut Game<'_>, key: &str, pressed: bool) {
    if !game.native_on(m::ID) {
        return;
    }
    let bound = game.native_mod_key(m::ID, m::KEY, m::DEFAULT_KEY);
    if key != bound {
        return;
    }
    let playing = game.native_playing();
    let updated = {
        let g = &*game;
        m::toggle_key(playing, pressed, |name| g.native_setting(m::ID, name))
    };
    if let Some(settings) = updated {
        for (k, v) in settings.stored() {
            game.native_set_setting(m::ID, k, &v);
        }
    }
}

pub(super) fn on_frame(game: &mut Game<'_>, elapsed: Duration) {
    let base_gamma = game.settings.gamma;
    let base_fullbright = game.settings.extras.fullbright;
    if !game.native_on(m::ID) {
        m::disable_mod();
        if let Some(r) = game.renderer.as_mut() {
            r.gamma = base_gamma;
            r.fullbright = base_fullbright;
        }
        return;
    }
    let (gamma, fullbright) = {
        let g = &*game;
        m::step(
            elapsed.as_secs_f32(),
            base_gamma,
            base_fullbright,
            |name| g.native_setting(m::ID, name),
        )
    };
    if let Some(r) = game.renderer.as_mut() {
        r.gamma = gamma;
        r.fullbright = fullbright;
    }
}

pub(super) fn on_hud(game: &mut Game<'_>, batch: &mut HudBatch) {
    if !game.native_on(m::ID) {
        return;
    }
    let Some((label, color)) = m::hud_banner() else {
        return;
    };
    let text_w = game.native_text_width(label);
    let box_w = text_w + 8.0;
    let box_h = 12.0;
    let x = ((batch.width - box_w) * 0.5).max(2.0);
    let y = (batch.height - 68.0).max(2.0);
    game.native_fill(batch, [x, y, box_w, box_h], m::BACKGROUND);
    game.native_text(batch, label, [x + 4.0, y + 2.0], color, true);
}

pub(super) fn on_list(mods: &mut Vec<NativeMod>) {
    mods.push(NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![[m::KEY.into(), m::KEY_LABEL.into(), m::DEFAULT_KEY.into()]],
    });
}

pub(super) fn on_settings(game: &mut Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    if mod_id != m::ID {
        return;
    }
    let g = &*game;
    let settings = m::settings(|key| g.native_setting(m::ID, key));
    buttons.extend(settings.labels());
}

pub(super) fn on_click(game: &mut Game<'_>, mod_id: &str, button: usize) {
    if mod_id != m::ID {
        return;
    }
    let mut settings = {
        let g = &*game;
        m::settings(|key| g.native_setting(m::ID, key))
    };
    settings.click(button);
    m::set_settings(settings);
    for (key, value) in settings.stored() {
        game.native_set_setting(m::ID, key, &value);
    }
}
