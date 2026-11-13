// A player left: save their knowledge and forget their menus.
fn pe_player_logged_out(player: u128) {
    crate::game::pe_glue::player_left(player);
}
