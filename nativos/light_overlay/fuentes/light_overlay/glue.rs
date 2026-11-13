// Glue between Horizon's `Game` and `mod_light_overlay` runtime state.
use crate::game::Game;
use crate::game::mod_light_overlay as m;
use crate::game::native_hooks::NativeMod;
use horizon_client::{
    blocks::Fluid,
    extras::Corner,
    extras_hud::Corners,
    gizmos::Line,
    hud::HudBatch,
};

pub(super) fn should_render_overlay(game: &Game<'_>) -> bool {
    if !game.native_on(m::ID) || game.player.is_none() {
        return false;
    }
    let s = m::settings(|k| game.native_setting(m::ID, k));
    s.active
}

pub(super) fn append_overlay_lines(game: &Game<'_>, lines: &mut Vec<Line>, width: f32) {
    let Some(p) = game.player.as_ref() else {
        m::set_counts(0, 0);
        return;
    };
    let s = m::settings(|k| game.native_setting(m::ID, k));
    if !s.active {
        m::set_counts(0, 0);
        return;
    }

    let px = p.position.x.floor() as i32;
    let py = p.position.y.floor() as i32;
    let pz = p.position.z.floor() as i32;
    let r = s.radius();
    let r_sq = r * r;
    let threshold = s.threshold();
    let facts_table = game.res.facts();
    let line_w = (width * 0.95).max(2.0);

    let mut red_count = 0u32;
    let mut yellow_count = 0u32;

    for dy in -6..=6 {
        let by = py + dy;
        for dz in -r..=r {
            let bz = pz + dz;
            for dx in -r..=r {
                if dx * dx + dz * dz > r_sq {
                    continue;
                }
                let bx = px + dx;

                // 1. Check solid spawnable top surface at (bx, by, bz)
                let below_state = game.world.block(bx, by, bz);
                if facts_table.is_air(below_state) {
                    continue;
                }
                let below = facts_table.get(below_state);
                // Direction.UP has ordinal 1 => bit (1 << 1) = 2 in sturdy_full
                let has_top_support =
                    below.full_collision || (below.sturdy_full & 2) != 0 || below.solid_render;
                if !has_top_support
                    || below.fluid != Fluid::None
                    || below.has_class("LeavesBlock")
                    || below.has_class("TransparentBlock")
                    || &*below.name == "minecraft:bedrock"
                    || &*below.name == "minecraft:barrier"
                {
                    continue;
                }

                // 2. Check clear spawn space at (bx, by + 1, bz)
                let above_state = game.world.block(bx, by + 1, bz);
                let above = facts_table.get(above_state);
                if above.full_collision
                    || above.solid_render
                    || above.fluid != Fluid::None
                    || above.waterlogged
                {
                    continue;
                }

                // 3. Read block & sky light at (bx, by + 1, bz)
                let packed = game.world.light(bx, by + 1, bz);
                let block_light = (packed & 0x0F).max(above.emission);
                let sky_light = (packed >> 4) & 0x0F;

                let safety = m::classify_spawn(block_light, sky_light, threshold);
                match safety {
                    m::SpawnSafety::AlwaysSpawnable => red_count += 1,
                    m::SpawnSafety::NightSpawnable => yellow_count += 1,
                    m::SpawnSafety::Safe if !s.show_safe => continue,
                    m::SpawnSafety::Safe => {}
                }

                let color = safety.argb();
                let draw_cross =
                    (s.mode == 0 || s.mode == 2) && safety != m::SpawnSafety::Safe;
                let draw_number = s.mode == 1 || s.mode == 2;

                if draw_cross {
                    for (start, end) in m::cross_segments(bx, by, bz) {
                        lines.push(Line {
                            start,
                            end,
                            color,
                            width: line_w,
                        });
                    }
                }
                if draw_number {
                    for (start, end) in m::number_segments(bx, by, bz, block_light) {
                        lines.push(Line {
                            start,
                            end,
                            color,
                            width: line_w,
                        });
                    }
                }
            }
        }
    }

    m::set_counts(red_count, yellow_count);
}

pub(super) fn on_key(game: &mut Game<'_>, key: &str, pressed: bool) {
    if !pressed || !game.native_on(m::ID) || !game.native_playing() {
        return;
    }
    let toggle_key = game.native_mod_key(m::ID, m::KEY_TOGGLE, m::KEY_TOGGLE_DEFAULT);
    if key != toggle_key {
        return;
    }
    let mut s = {
        let g = &*game;
        m::settings(|k| g.native_setting(m::ID, k))
    };
    s.active = !s.active;
    m::set_settings(s);
    for (k, v) in s.stored() {
        game.native_set_setting(m::ID, k, &v);
    }
    let state_str = if s.active { "ON" } else { "OFF" };
    game.native_chat(&format!(
        "[Light Overlay] {state_str} ({}, r={})",
        m::Settings::mode_name(s.mode),
        s.radius()
    ));
}

pub(super) fn on_hud(game: &mut Game<'_>, batch: &mut HudBatch, corners: &mut Corners) {
    if !game.native_on(m::ID) {
        return;
    }
    let s = {
        let g = &*game;
        m::settings(|k| g.native_setting(m::ID, k))
    };
    if !s.active || !s.show_hud || !game.native_corner_shown(Corner::TopLeft) {
        return;
    }
    let (red, yellow) = m::counts();
    let text = format!(
        "Light Overlay [F7]: {} (Red: {red}, Yellow: {yellow})",
        m::Settings::mode_name(s.mode)
    );
    let w = game.native_text_width(&text) + 8.0;
    let h = 14.0;
    let (x, y) = corners.place(Corner::TopLeft, w, h);
    game.native_fill(batch, [x, y, w, h], m::BACKGROUND);
    game.native_text(batch, &text, [x + 4.0, y + 3.0], m::HUD_ACTIVE_COLOR, true);
}

pub(super) fn on_list(mods: &mut Vec<NativeMod>) {
    mods.push(NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![[
            m::KEY_TOGGLE.into(),
            m::KEY_TOGGLE_LABEL.into(),
            m::KEY_TOGGLE_DEFAULT.into(),
        ]],
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
