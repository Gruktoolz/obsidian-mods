fn rider_hud_layer_pre(
    game: &mut crate::game::Game<'_>,
    layer: &str,
    batch: &mut horizon_client::hud::HudBatch,
    canceled: &mut bool,
) {
    use crate::game::rider_hud as m;
    if !m::is_enabled(|| game.native_setting(m::ID, "enabled")) {
        return;
    }

    if layer == "minecraft:vehicle_health" {
        if m::is_show_hunger(|| game.native_setting(m::ID, "hunger")) {
            // Cancel vanilla mount health at h - 39 to leave that row free for player hunger
            *canceled = true;

            // Draw mount health shifted up by VEHICLE_HEALTH_Y_OFFSET (10px) to h - 49
            if let Some(i) = game.mount_index() {
                let mob = &game.mobs[i];
                let hearts = horizon_client::hud_layout::vehicle_max_hearts(mob.attrs.max_health);
                if hearts > 0 {
                    let wi = batch.width as i32;
                    let hi = batch.height as i32 - m::VEHICLE_HEALTH_Y_OFFSET;
                    let draws = horizon_client::hud_layout::vehicle_health(wi, hi, hearts, mob.health);
                    horizon_client::hud_layout::paint(batch, &game.gui.atlas, &draws);
                }
            }
        }
    } else if layer == "minecraft:contextual_info_bar" {
        if m::is_show_exp(|| game.native_setting(m::ID, "exp")) {
            let is_jumpable = game.jumpable_mount();
            let is_jumping = game.mount_jumping();
            let has_waypoints = game.waypoints.has_waypoints();

            if m::should_show_experience_instead_of_jump(is_jumpable, is_jumping, has_waypoints, game.creative) {
                // Cancel the empty jump bar and draw the experience progress bar and level
                *canceled = true;

                if game.player.is_some() {
                    let wi = batch.width as i32;
                    let hi = batch.height as i32;
                    let atlas = &game.gui.atlas;
                    let width = |s: &str| horizon_client::hud::HudBatch::text_width(atlas, s) as i32;
                    let draws = horizon_client::hud_layout::experience(
                        wi,
                        hi,
                        game.experience.progress,
                        game.experience.level,
                        true, // draw the full experience progress bar
                        &width,
                    );
                    horizon_client::hud_layout::paint(batch, atlas, &draws);
                }
            }
        }
    }
}

