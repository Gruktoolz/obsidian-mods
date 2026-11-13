// The lightmap's gamma this frame: the mod's while it is on, else the slider's.
fn fullbright_frame(game: &mut crate::game::Game<'_>) {
    use crate::game::fullbright_gamma as fb;
    let on = game.native_on(fb::ID);
    let state = {
        let game = &*game;
        fb::settings(|key| game.native_setting(fb::ID, key))
    };
    let gamma = match state.active() {
        Some(custom) if on => custom,
        _ => game.settings.gamma,
    };
    if let Some(renderer) = game.renderer.as_mut() {
        renderer.gamma = gamma;
    }
}
