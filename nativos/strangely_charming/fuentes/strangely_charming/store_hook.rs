fn hook(game: &mut crate::game::Game<'_>, location: &str, rgba: &[u8]) {
    if let Some(skin) = horizon_client_render::strangely_charming_skin::bind(location, rgba) {
        for (kind, pixels) in &skin.textures {
            game.store_skin(
                horizon_client_render::strangely_charming_skin::texture_name(location, kind),
                pixels.clone(),
            );
        }
    }
}
