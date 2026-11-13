// Glue between Horizon's `Game` and `mod_appleskin` runtime state.
// Renders the exact 1:1 Java AppleSkin saturation overlay on player food drumsticks.
use crate::game::Game;
use crate::game::mod_appleskin as m;
use crate::game::native_hooks::NativeMod;
use horizon_client::hud::HudBatch;

pub(super) fn on_hud(
    game: &Game<'_>,
    batch: &mut HudBatch,
) {
    if !game.native_on(m::ID) {
        return;
    }
    // Only draw during gameplay when alive and not in spectator/creative
    if !game.native_playing() || game.spectator() || game.creative {
        return;
    }
    let Some(p) = &game.player else {
        return;
    };

    let settings = {
        let g = game;
        m::settings(|key| g.native_setting(m::ID, key))
    };

    if !settings.saturation_overlay {
        return;
    }

    let current_food = p.health.food.food;
    let current_sat = p.health.food.saturation;

    // Check held food in main hand or off hand
    let items_table = game.items();
    let held_food = game
        .inventory
        .get(game.selected)
        .and_then(Option::as_ref)
        .and_then(|st| items_table.get(&st.item).and_then(|info| info.food))
        .or_else(|| {
            game.offhand
                .as_ref()
                .and_then(|st| items_table.get(&st.item).and_then(|info| info.food))
        });

    let (x_right, y_base) = m::drumstick_origin(batch.width, batch.height);

    // 1. Draw 1:1 AppleSkin saturation overlay for current saturation
    if current_sat > 0.0 {
        for n in 0..10 {
            let level = m::drumstick_saturation_level(n, current_sat);
            let pixels = level.pixels();
            if pixels.is_empty() {
                continue;
            }
            let (x, y) = m::drumstick_pos(x_right, y_base, n);
            for &(px, py, color) in pixels {
                game.native_fill(batch, [x + px, y + py, 1.0, 1.0], color);
            }
        }
    }

    // 2. Draw 1:1 AppleSkin preview saturation overlay for held food (pulsing)
    if let Some((nutrition, sat_mod)) = held_food {
        let target_sat = m::calculate_saturation_restore(nutrition, sat_mod, current_food, current_sat);
        if target_sat > current_sat {
            let pulse = m::pulse_alpha(game.started.elapsed().as_millis() as u64);
            let alpha_byte = (pulse * 255.0) as u8;

            for n in 0..10 {
                let prev_level = m::drumstick_saturation_level(n, target_sat);
                let curr_level = m::drumstick_saturation_level(n, current_sat);

                // Only render preview pixels on shanks where saturation is newly added/increased
                if prev_level != m::SaturationLevel::None && prev_level != curr_level {
                    let (x, y) = m::drumstick_pos(x_right, y_base, n);
                    for &(px, py, base_color) in prev_level.pixels() {
                        let color = ((alpha_byte as u32) << 24) | (base_color & 0x00FF_FFFF);
                        game.native_fill(batch, [x + px, y + py, 1.0, 1.0], color);
                    }
                }
            }
        }
    }
}

pub(super) fn on_list(mods: &mut Vec<NativeMod>) {
    mods.push(NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: Vec::new(),
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
