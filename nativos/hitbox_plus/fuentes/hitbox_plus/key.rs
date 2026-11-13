fn hitbox_plus_key(
    game: &mut crate::game::Game<'_>,
    key: &str,
    pressed: bool,
) {
    use horizon_client::official_hitbox_plus as m;
    if !pressed || !game.native_on(m::ID) || !game.native_playing() {
        return;
    }
    let bound = game.native_mod_key(m::ID, m::KEY, m::DEFAULT_KEY);
    if key == bound {
        let enabled = m::toggle();
        game.native_chat(&format!(
            "HitBox+ {}",
            if enabled { "Enabled" } else { "Disabled" }
        ));
    }
}
