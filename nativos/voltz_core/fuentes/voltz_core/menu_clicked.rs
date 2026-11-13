// A click in a menu a mod opened: the ME terminal's.
fn voltz_menu_clicked(game: &mut crate::game::Game<'_>, uuid: u128, slot: i32, button: i32, mode: i32) {
    crate::game::voltz_glue::menu_clicked(game, uuid, slot, button, mode);
}
