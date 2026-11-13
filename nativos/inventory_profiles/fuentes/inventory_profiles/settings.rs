// Hook helper: native_settings
fn inventory_profiles_settings(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    crate::game::mod_inventory_profiles_glue::on_settings(game, mod_id, buttons);
}
