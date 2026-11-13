// A click on the settings page: 0-2 change a setting, 3 opens the timer's
// folder in the computer's file manager, 4 resets the timer.
fn speedrun_timer_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::speedrun_timer as m;
    if mod_id != m::ID {
        return;
    }
    if button == 3 {
        let dir = game.native_data_dir(m::ID);
        match m::open_folder(&dir) {
            Ok(()) => game.native_chat(&format!("Opening {}", dir.display())),
            Err(_) => game.native_chat(&format!("The timer's folder is {}", dir.display())),
        }
        return;
    }
    if button == 4 {
        m::reset();
        game.native_chat("Speedrun timer reset");
        return;
    }
    let mut settings = m::settings(|key| game.native_setting(m::ID, key));
    settings.click(button);
    m::set_settings(settings);
    for (key, value) in settings.stored() {
        game.native_set_setting(m::ID, key, &value);
    }
}
