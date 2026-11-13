// The mod's settings page: one button per option, then reset.
fn map_preview_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::map_preview as m;
    if mod_id != m::ID {
        return;
    }
    let game = &*game;
    buttons.extend(m::settings(|key| game.native_setting(m::ID, key)).labels());
}
