// The mod's settings page: the WebAssembly Blockscope's five buttons.
fn blockscope_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::native_blockscope as m;
    if mod_id != m::ID {
        return;
    }
    let game = &*game;
    buttons.extend(crate::game::native_blockscope_glue::settings(game).labels());
}
