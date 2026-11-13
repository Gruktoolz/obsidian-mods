// A click in the transmutation or alchemical bag menu.
fn pe_menu_clicked(game: &mut crate::game::Game<'_>, uuid: u128, slot: i32, button: i32, mode: i32) {
    crate::game::pe_glue::menu_clicked(game, uuid, slot, button, mode);
}
