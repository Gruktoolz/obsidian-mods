fn movement_status_hud( game: &mut crate::game::Game<'_>, batch: &mut horizon_client::hud::HudBatch, corners: &mut horizon_client::extras_hud::Corners, ) {
    use crate::game::mod_movement_status as m;
    use horizon_client::extras::Corner;
    use winit::keyboard::KeyCode;

    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::BottomLeft) { return; }

    // Extract everything we need from the player first so the borrow ends
    let (sprinting, crouching, flying, fall_flying) = {
        let Some(p) = game.player.as_ref() else { return; };
        (p.sprinting, p.local_crouching, p.flying, p.fall_flying)
    };

    // Now game is free again — these need &mut self
    let sprint_logical = game.native_mapping("key.sprint").map(|(held, _)| held).unwrap_or(false);
    let sneak_logical = game.native_mapping("key.sneak").map(|(held, _)| held).unwrap_or(false);

    let sprint_phys = game.physical_keys.contains(&KeyCode::ControlLeft);
    let sneak_phys = game.physical_keys.contains(&KeyCode::ShiftLeft);

    let toggle_sprint = game.settings.toggle_sprint;
    let toggle_sneak = game.settings.toggle_crouch;

    let sprint_held = sprint_phys || (sprint_logical && !toggle_sprint);
    let sneak_held = sneak_phys || (sneak_logical && !toggle_sneak);

    let lines = m::status_lines( sprinting, crouching, flying, fall_flying, sprint_held, sneak_held, toggle_sprint, toggle_sneak, );

    if lines.is_empty() { return; }

    let w = lines
        .iter()
        .map(|l| game.native_text_width(l))
        .fold(0.0, f32::max)
        + 8.0;
    let h = 10.0 * lines.len() as f32 + 4.0;

    let (x, y) = corners.place(Corner::BottomLeft, w, h);

    if m::background(|| game.native_setting(m::ID, "background")) { game.native_fill(batch, [x, y, w, h], m::BACKGROUND_COLOR); }

    for (i, line) in lines.iter().enumerate() {
        game.native_text(
            batch,
            line,
            [x + 4.0, y + 2.0 + 10.0 * i as f32],
            m::INK,
            true,
        );
    }
}
