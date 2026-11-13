// Hook helper: native_frame
fn inventory_profiles_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    crate::game::mod_inventory_profiles_glue::on_frame(game, elapsed);
}
