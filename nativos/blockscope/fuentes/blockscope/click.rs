// A click on the mod's settings page: the next value, kept where the
// WebAssembly mod kept it.
fn blockscope_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::native_blockscope as m;
    if mod_id != m::ID {
        return;
    }
    crate::game::native_blockscope_glue::click(game, button);
}
