// Fullbright's settings page.
fn fullbright_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::fullbright_gamma as fb;
    if mod_id != fb::ID {
        return;
    }
    let game = &*game;
    let settings = fb::settings(|key| game.native_setting(fb::ID, key));
    buttons.extend(settings.labels());
}
