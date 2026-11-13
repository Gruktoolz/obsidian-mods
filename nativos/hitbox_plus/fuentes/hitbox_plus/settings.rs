fn hitbox_plus_settings(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    use horizon_client::official_hitbox_plus as m;
    if mod_id != m::ID {
        return;
    }
    let _ = game;
    buttons.push(format!(
        "Hitbox+: {}",
        if m::is_enabled() { "ON" } else { "OFF" }
    ));
    buttons.push(format!(
        "Color: {}",
        m::STYLES[(m::STYLE_MODE.load(std::sync::atomic::Ordering::Relaxed) as usize) % m::STYLES.len()]
    ));
    buttons.push(format!(
        "Thickness: {}",
        m::THICKNESS_LABELS[(m::THICKNESS_MODE.load(std::sync::atomic::Ordering::Relaxed) as usize) % m::THICKNESS_LABELS.len()]
    ));
    buttons.push(format!(
        "Eye Ray: {}",
        if m::show_eye_ray() { "ON" } else { "OFF" }
    ));
    buttons.push(format!(
        "Sprint Display: {}",
        if m::show_sprint_display() { "ON" } else { "OFF" }
    ));
    buttons.push(format!(
        "Sprint Pos: {}",
        m::SPRINT_CORNER_NAMES[(m::sprint_corner() as usize) % 4]
    ));
}
