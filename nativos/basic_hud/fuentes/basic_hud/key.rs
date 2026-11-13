// Hotkey to open the visual in-game HUD editor over the playing world.
fn basic_hud_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::{basic_hud as m, basic_hud_editor as editor};

    if !pressed {
        return;
    }
    let chosen = game.native_mod_key(m::ID, "layout_editor", "key.keyboard.right.shift");
    if key != chosen {
        return;
    }
    if !game.native_playing() || !game.native_window() || !game.native_on(m::ID) || editor::active() {
        return;
    }

    game.open_options_at(false, horizon_client::options_ui::Page::Mods);
    game.options_to_game = true;
    game.capture(false);
    editor::open(game);
}
