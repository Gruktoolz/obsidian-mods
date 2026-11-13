// A click on the mod's settings page: the heads on or off, kept.
fn chatheads_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::chat::native_chatheads as m;
    if mod_id != m::ID {
        return;
    }
    crate::game::chat::native_chatheads_glue::click(game, button);
}
