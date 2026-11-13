// The lightmap's gamma this frame: Fullbright's while it is on, the slider's
// once after it goes off.
fn fullbright_nofog_frame(game: &mut crate::game::Game<'_>) {
    use crate::game::fullbright_nofog as m;
    let settings = {
        let game = &*game;
        m::settings(|key| game.native_setting(m::ID, key))
    };
    let on = settings.fullbright && game.native_on(m::ID);
    if let Some(gamma) = m::frame_gamma(on, game.settings.gamma) {
        if let Some(renderer) = game.renderer.as_mut() {
            renderer.gamma = gamma;
        }
    }
}
