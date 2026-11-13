fn rider_hud_health_hud(
    game: &mut crate::game::Game<'_>,
    inputs: &horizon_client::hud_layout::HealthInputs,
    wi: i32,
    hi: i32,
    draws: &mut Option<Vec<horizon_client::hud_layout::GuiDraw>>,
) {
    use crate::game::rider_hud as m;
    if !m::is_enabled(|| game.native_setting(m::ID, "enabled")) {
        return;
    }
    if !m::is_show_hunger(|| game.native_setting(m::ID, "hunger")) {
        return;
    }

    // Intervene only when riding a living vehicle that has health (inputs.vehicle_hearts > 0).
    // When on foot (vehicle_hearts == 0), vanilla already renders player health and hunger correctly.
    if inputs.vehicle_hearts > 0 {
        // Request the standard health layout with vehicle_hearts = 0 so that player hunger is rendered at y = h - 39.
        let foot_inputs = horizon_client::hud_layout::HealthInputs {
            tick: inputs.tick,
            millis: inputs.millis,
            health: inputs.health,
            max_health: inputs.max_health,
            absorption: inputs.absorption,
            armor: inputs.armor,
            food: inputs.food,
            saturation: inputs.saturation,
            kind: inputs.kind,
            regeneration: inputs.regeneration,
            hunger: inputs.hunger,
            hardcore: inputs.hardcore,
            damage_cooldown: inputs.damage_cooldown,
            air: inputs.air,
            max_air: inputs.max_air,
            under_water: inputs.under_water,
            vehicle_hearts: 0,
        };

        let mut layout_draws = game.native_health_hud_state().layout(&foot_inputs, wi, hi);

        // If underwater or holding breath, air bubbles were placed at y = h - 49 (since vehicle_hearts = 0).
        // Shift air bubbles up above the vehicle hearts (which take vehicle_heart_rows * 10 vertical pixels).
        let shift_y = m::air_bubbles_shift_y(inputs.vehicle_hearts);
        if shift_y > 0 && (inputs.under_water || inputs.air < inputs.max_air) {
            for d in &mut layout_draws {
                if let horizon_client::hud_layout::GuiDraw::Sprite { sprite, y, .. } = d {
                    if sprite.starts_with("hud/air") {
                        *y -= shift_y;
                    }
                }
            }
        }

        *draws = Some(layout_draws);
    }
}

