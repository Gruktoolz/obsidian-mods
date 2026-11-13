// Hook helper: native_key
fn inventory_profiles_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    crate::game::mod_inventory_profiles_glue::on_key(game, key, pressed);
}
