// A click on one of HUD+'s Options > Mods buttons.
fn official_hud_plus_options_click(mod_id: &str, button: usize) {
    crate::game::official_hud_plus_glue::options_click(mod_id, button);
}
