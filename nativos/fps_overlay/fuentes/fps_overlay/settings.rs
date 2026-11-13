// The mod's settings page: the WebAssembly mod's buttons.
fn official_fps_overlay_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::official_fps_overlay as m;
    if mod_id != m::ID {
        return;
    }
    let game = &*game;
    buttons.extend(m::settings(|key| game.native_setting(m::ID, key)).labels());
}
