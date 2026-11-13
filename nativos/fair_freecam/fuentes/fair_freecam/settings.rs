// Fair Freecam's settings page.
fn freecam_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::fair_freecam as fc;
    if mod_id != fc::ID {
        return;
    }
    let game = &*game;
    let settings = fc::settings(|key| game.native_setting(fc::ID, key));
    buttons.extend(settings.labels());
}
