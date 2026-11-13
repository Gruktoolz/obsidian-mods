// A ProjectE menu closed.
fn pe_menu_closed(uuid: u128, window: i32) {
    crate::game::pe_glue::menu_closed(uuid, Some(window));
}
