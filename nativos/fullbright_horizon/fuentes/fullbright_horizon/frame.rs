// Each frame: update Fullbright transition and sync renderer lightmap / gamma.
fn fullbright_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    crate::game::mod_fullbright_glue::on_frame(game, elapsed);
}
