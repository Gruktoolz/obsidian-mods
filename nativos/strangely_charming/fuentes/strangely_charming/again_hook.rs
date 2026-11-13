fn hook(game: &mut crate::game::Game<'_>, location: &str) {
    if let Some(skin) = horizon_client_render::strangely_charming_skin::get(location) {
        for (kind, pixels) in &skin.textures {
            let name = horizon_client_render::strangely_charming_skin::texture_name(location, kind);
            if !game.gui.atlas.sprites.contains_key(&name) {
                game.store_skin(name.clone(), pixels.clone());
            }
            game.skin_downloads.borrow_mut().drawn(&name);
        }
    }
}
