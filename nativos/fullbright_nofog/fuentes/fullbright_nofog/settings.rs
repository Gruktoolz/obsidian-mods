// The mod's settings page.
fn fullbright_nofog_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::fullbright_nofog as m;
    if mod_id != m::ID {
        return;
    }
    let game = &*game;
    let settings = m::settings(|key| game.native_setting(m::ID, key));
    buttons.extend(settings.labels());
}
