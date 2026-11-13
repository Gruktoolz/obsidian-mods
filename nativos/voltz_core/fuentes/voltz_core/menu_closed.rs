// A mod menu closed: forget the player's ME terminal session.
fn voltz_menu_closed(uuid: u128) {
    crate::game::voltz_glue::menu_closed(uuid);
}
