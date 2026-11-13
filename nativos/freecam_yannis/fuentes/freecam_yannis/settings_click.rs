fn freecam_settings_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::freecam_yannis as m;

    if mod_id != m::ID {
        return;
    }
    let mut settings = m::Settings::load(|k| game.native_setting(m::ID, k));
    fn next(current: f64, values: &[f64]) -> f64 {
        values
            .iter()
            .copied()
            .find(|v| *v > current)
            .unwrap_or(values[0])
    }
    match button {
        0 => settings.speed = next(settings.speed, &[2.0, 5.0, 10.0, 20.0, 40.0, 80.0]),
        1 => settings.distance = next(settings.distance, &[4.0, 8.0, 16.0, 28.0, 40.0, 60.0, 80.0]),
        2 => {
            settings.height = next(
                settings.height,
                &[2.0, 6.0, 12.0, 18.0, 28.0, 40.0, 60.0, 80.0],
            )
        }
        3 => settings = m::Settings::default(),
        4 => {
            m::STATE.with(|s| s.borrow_mut().slots = [None; 9]);
            return;
        }
        _ => return,
    }
    for (k, v) in [
        ("speed", settings.speed),
        ("distance", settings.distance),
        ("height", settings.height),
    ] {
        game.native_set_setting(m::ID, k, &v.to_string());
    }
    m::STATE.with(|s| s.borrow_mut().settings = Some(settings));
}
