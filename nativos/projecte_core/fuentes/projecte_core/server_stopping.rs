// The server stops: save every player's knowledge.
fn pe_server_stopping() {
    crate::game::pe_glue::world_closing();
}
