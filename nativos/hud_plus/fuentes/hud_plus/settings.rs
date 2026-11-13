// HUD+'s buttons on its Options > Mods page.
fn official_hud_plus_page(mod_id: &str, buttons: &mut Vec<String>) {
    crate::game::official_hud_plus_glue::options_buttons(mod_id, buttons);
}
