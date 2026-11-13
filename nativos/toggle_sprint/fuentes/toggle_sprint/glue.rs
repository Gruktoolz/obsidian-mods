// Glue between Horizon's `Game` and `mod_toggle_sprint` runtime state.
use crate::game::{Game, Screen};
use crate::game::mod_toggle_sprint as m;
use crate::game::native_hooks::NativeMod;
use horizon_client::{
    extras::Corner,
    extras_hud::Corners,
    hud::HudBatch,
};
use std::time::Duration;
use winit::keyboard::KeyCode;

pub(super) fn on_frame(game: &mut Game<'_>, _elapsed: Duration) {
    if !game.native_on(m::ID) {
        return;
    }
    let s = {
        let g = &*game;
        m::settings(|k| g.native_setting(m::ID, k))
    };
    if !s.enabled {
        return;
    }

    let playing = game.screen == Screen::Playing && game.sleeping.is_none();
    let phys_sprint = game.physical_keys.contains(&KeyCode::ControlLeft);
    let phys_sneak = game.physical_keys.contains(&KeyCode::ShiftLeft);

    // Maintain toggled sprint in `game.keys` while playing
    if playing && (s.sprint_toggled || phys_sprint) {
        game.keys.insert(KeyCode::ControlLeft);
    } else if !phys_sprint {
        game.keys.remove(&KeyCode::ControlLeft);
    }

    // Maintain toggled sneak in `game.keys` ONLY while playing (never inside inventory/chest screens!)
    if playing && (s.sneak_toggled || phys_sneak) {
        game.keys.insert(KeyCode::ShiftLeft);
    } else if !phys_sneak {
        game.keys.remove(&KeyCode::ShiftLeft);
    }

    // Apply Creative Fly Speed Boost when flying in Creative mode
    if let Some(p) = game.player.as_mut() {
        if p.flying && !p.spectator {
            let sprint_active = s.sprint_toggled || phys_sprint || p.sprinting;
            p.flying_speed = m::compute_flying_speed(&s, sprint_active);
        }
    }
}

pub(super) fn on_key(game: &mut Game<'_>, key: &str, pressed: bool) {
    if !pressed || !game.native_on(m::ID) || !game.native_playing() {
        return;
    }
    let mut s = {
        let g = &*game;
        m::settings(|k| g.native_setting(m::ID, k))
    };
    if !s.enabled {
        return;
    }

    let sprint_key = game.native_mod_key(m::ID, m::KEY_SPRINT, m::KEY_SPRINT_DEFAULT);
    let sneak_key = game.native_mod_key(m::ID, m::KEY_SNEAK, m::KEY_SNEAK_DEFAULT);
    let vanilla_sprint_key = game.settings.keys.get("key.sprint").to_owned();

    let mut changed = false;
    if key == sprint_key || (s.tap_sprint_key_toggles && key == vanilla_sprint_key) {
        s.sprint_toggled = !s.sprint_toggled;
        changed = true;
    } else if key == sneak_key {
        s.sneak_toggled = !s.sneak_toggled;
        changed = true;
    }

    if changed {
        m::set_settings(s);
        for (k, v) in s.stored() {
            game.native_set_setting(m::ID, k, &v);
        }
    }
}

pub(super) fn on_hud(game: &mut Game<'_>, batch: &mut HudBatch, corners: &mut Corners) {
    if !game.native_on(m::ID) || game.player.is_none() {
        return;
    }
    let s = {
        let g = &*game;
        m::settings(|k| g.native_setting(m::ID, k))
    };
    if !s.enabled || !s.show_hud {
        return;
    }

    let corner = match s.position % 4 {
        0 => Corner::BottomLeft,
        1 => Corner::TopLeft,
        2 => Corner::TopRight,
        _ => Corner::BottomRight,
    };
    if !game.native_corner_shown(corner) {
        return;
    }

    let mov = {
        let Some(p) = game.player.as_ref() else {
            return;
        };
        m::MovementSnapshot {
            flying: p.flying,
            fall_flying: p.fall_flying,
            riding: game.riding.is_some(),
            swimming: p.swimming,
            sprinting: p.sprinting,
            crouching: p.local_crouching,
            sprint_key_held: game.physical_keys.contains(&KeyCode::ControlLeft),
            sneak_key_held: game.physical_keys.contains(&KeyCode::ShiftLeft),
        }
    };

    let Some(label) = m::format_status_text(&s, &mov) else {
        return;
    };

    let w = game.native_text_width(&label) + 8.0;
    let h = 14.0;
    let (x, y) = corners.place(corner, w, h);
    if s.background {
        game.native_fill(batch, [x, y, w, h], m::BACKGROUND);
    }
    game.native_text(batch, &label, [x + 4.0, y + 3.0], s.text_color(), true);
}

pub(super) fn on_list(mods: &mut Vec<NativeMod>) {
    mods.push(NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![
            [
                m::KEY_SPRINT.into(),
                m::KEY_SPRINT_LABEL.into(),
                m::KEY_SPRINT_DEFAULT.into(),
            ],
            [
                m::KEY_SNEAK.into(),
                m::KEY_SNEAK_LABEL.into(),
                m::KEY_SNEAK_DEFAULT.into(),
            ],
        ],
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
