// The mod's settings page: one button, the heads on or off.
fn chatheads_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::chat::native_chatheads as m;
    if mod_id != m::ID {
        return;
    }
    buttons.extend(crate::game::chat::native_chatheads_glue::settings(game).labels());
}
