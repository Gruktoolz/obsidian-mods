// Hook helper: native_settings_click
fn inventory_profiles_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    crate::game::mod_inventory_profiles_glue::on_click(game, mod_id, button);
}
