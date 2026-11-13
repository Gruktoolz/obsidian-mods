fn freecam_settings(game: &crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::freecam_yannis as m;

    if mod_id != m::ID {
        return;
    }
    let s = m::Settings::load(|k| game.native_setting(m::ID, k));
    buttons.extend([
        format!("Camera speed: {} blocks/s", s.speed),
        format!("Follow distance: {} blocks", s.distance),
        format!("Follow height: {} blocks", s.height),
        "Reset camera settings".into(),
        "Clear tripod slots".into(),
    ]);
}
