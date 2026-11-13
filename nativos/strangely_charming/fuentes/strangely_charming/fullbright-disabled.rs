fn hook(game: &mut crate::game::Game<'_>) {
    if !horizon_mods::evanders_config::enabled("fullbright") {
        game.settings.extras.fullbright = false;
    }
}
