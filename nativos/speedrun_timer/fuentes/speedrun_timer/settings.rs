// The mod's settings page: corner, shown, personal best, folder, reset.
fn speedrun_timer_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::speedrun_timer as m;
    if mod_id != m::ID {
        return;
    }
    let game = &*game;
    buttons.extend(m::settings(|key| game.native_setting(m::ID, key)).labels());
}
