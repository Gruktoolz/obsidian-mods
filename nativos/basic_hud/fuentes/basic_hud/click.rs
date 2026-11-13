// A click on the mod's settings page, kept at once.
fn basic_hud_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::{basic_hud as m, basic_hud_editor as editor};
    if mod_id != m::ID {
        return;
    }
    if button == 0 {
        editor::open(game);
        return;
    }
    let mut current = m::config(|| game.native_setting(m::ID, "config"), m::Config::clone);
    current.click(button - 1);
    m::commit_verified(
        &current,
        || game.native_setting(m::ID, "config"),
        |line| game.native_set_setting(m::ID, "config", line),
    );
}
